use crate::devices::launch_control_xl_mk2::{LCXL2Driver, LCXL2Position};
use crate::devices::launchpad_mini_mk3::{LPM3Driver, LPM3Position, LPM3Visual};
use crate::{MidiDriver, MidiDriverError, MidiPhysicalState};
use color_lib::utils::hsv_to_rgb;
use color_lib::ColorHSVA;
use ndarray::{arr2, s, Array1, Array2};
use std::time::Instant;

pub struct CubeDemo {
    last_time: Instant,
    t: f64,

    size: f64,
    ax: f64,
    ay: f64,
    az: f64,
}

impl Default for CubeDemo {
    fn default() -> Self {
        Self {
            last_time: Instant::now(),
            t: 0.,
            size: 3.,
            ax: 0.,
            ay: 0.,
            az: 0.,
        }
    }
}

impl CubeDemo {
    pub fn call_loop(
        &mut self,
        lpm3: &mut LPM3Driver,
        lcxl2: &mut LCXL2Driver,
    ) -> Result<(), MidiDriverError> {
        let now = Instant::now();
        let delta = now - self.last_time;
        self.last_time = now;

        let dt = delta.as_secs_f64();

        let _ = lpm3.read()?;
        let _ = lcxl2.read()?;

        let grid = self.get_cube_pixel_grid();

        lpm3.clear()?;
        for y in 0..9 {
            for x in 0..9 {
                let v = grid[[x, y]].powi(3);
                let color = hsv_to_rgb(&ColorHSVA::new_hsv(
                    (self.t / 15.0 + (y + x) as f64 * 0.02) as f32,
                    1.0,
                    v as f32,
                ));
                let p = (y as u8 + 1) * 10 + x as u8 + 1;
                lpm3.add(LPM3Visual::RGB(
                    LPM3Position::Raw(p),
                    (color.get_red() * 127.999).floor() as u8,
                    (color.get_green() * 127.999).floor() as u8,
                    (color.get_blue() * 127.999).floor() as u8,
                ))?
            }
        }
        lpm3.push()?;

        let MidiPhysicalState::Analog8(rx) = lcxl2.get_position_state(LCXL2Position::Slider(1))?
        else {
            unreachable!()
        };
        let MidiPhysicalState::Analog8(ry) = lcxl2.get_position_state(LCXL2Position::Slider(2))?
        else {
            unreachable!()
        };
        let MidiPhysicalState::Analog8(rz) = lcxl2.get_position_state(LCXL2Position::Slider(3))?
        else {
            unreachable!()
        };

        self.ax += (rx as f64 / 127.) * dt;
        self.ay += (ry as f64 / 127.) * dt;
        self.az += (rz as f64 / 127.) * dt;

        self.t += dt;

        Ok(())
    }

    fn get_rotation_matrix(&self) -> Array2<f64> {
        let rx = arr2(&[
            [1., 0., 0.],
            [0., self.ax.cos(), -self.ax.sin()],
            [0., self.ax.sin(), self.ax.cos()],
        ]);
        let ry = arr2(&[
            [self.ay.cos(), 0., self.ay.sin()],
            [0., 1., 0.],
            [-self.ay.sin(), 0., self.ay.cos()],
        ]);
        let rz = arr2(&[
            [self.az.cos(), -self.az.sin(), 0.],
            [self.az.sin(), self.az.cos(), 0.],
            [0., 0., 1.],
        ]);
        rx.dot(&ry).dot(&rz)
    }

    fn get_cube_points(&self) -> Array2<f64> {
        let points = arr2(&[
            [-1., -1., -1.],
            [-1., -1., 1.],
            [-1., 1., -1.],
            [-1., 1., 1.],
            [1., -1., -1.],
            [1., -1., 1.],
            [1., 1., -1.],
            [1., 1., 1.],
        ]);
        (points * self.size).dot(&self.get_rotation_matrix())
    }

    fn max_pixel_on_grid(grid: &mut Array2<f64>, x: i8, y: i8, v: f64) {
        if x < -4 || y < -4 || x > 4 || y > 4 {
            return;
        }
        let ux = (x + 4) as usize;
        let uy = (y + 4) as usize;

        if v > 1.0 {
            panic!("invalid value in cube pixel grid... ({}, {})", x, y)
        }

        if v > grid[[ux, uy]] {
            grid[[ux, uy]] = v;
        }
    }

    fn draw_line(grid: &mut Array2<f64>, a: &Array1<f64>, b: &Array1<f64>) {
        let mut x0 = a[0];
        let mut y0 = a[1];
        let mut x1 = b[0];
        let mut y1 = b[1];

        let steep = (y1 - y0).abs() > (x1 - x0).abs();

        if steep {
            let t = x0;
            x0 = y0;
            y0 = t;
            let t = x1;
            x1 = y1;
            y1 = t;
        }
        if x0 > x1 {
            let t = x0;
            x0 = x1;
            x1 = t;
            let t = y0;
            y0 = y1;
            y1 = t;
        }

        let dx = x1 - x0;
        let dy = y1 - y0;

        let gradient = match dx == 0.0 {
            true => 1.0,
            false => dy / dx,
        };

        let xend = x0.floor();
        let yend = y0 + gradient * (xend - x0);
        let xgap = 1. - (x0 - xend);
        let xpxl1 = xend;
        let ypxl1 = yend.floor();
        let yend_fract = yend - yend.floor();
        if steep {
            //grid[[ypxl1 as usize, xpxl1 as usize]] = (1. - yend.fract()) * xgap;
            //grid[[ypxl1 as usize + 1, xpxl1 as usize]] = yend.fract() * xgap;
            // trace!("draw_line 1 steep");
            // trace!("xgap: {}", xgap);
            // trace!("yend.fract(): {}", yend.fract());
            Self::max_pixel_on_grid(grid, ypxl1 as i8, xpxl1 as i8, (1. - yend_fract) * xgap);
            Self::max_pixel_on_grid(grid, ypxl1 as i8 + 1, xpxl1 as i8, yend_fract * xgap);
        } else {
            // trace!("draw_line 1 !steep");
            // trace!("xgap: {}", xgap);
            // trace!("yend.fract(): {}", yend.fract());
            //grid[[xpxl1 as usize, ypxl1 as usize]] = (1. - yend.fract()) * xgap;
            //grid[[xpxl1 as usize, ypxl1 as usize + 1]] = yend.fract() * xgap;
            Self::max_pixel_on_grid(grid, xpxl1 as i8, ypxl1 as i8, (1. - yend_fract) * xgap);
            Self::max_pixel_on_grid(grid, xpxl1 as i8, ypxl1 as i8 + 1, yend_fract * xgap);
        }

        let mut intery = yend + gradient;

        let xend = x1.ceil();
        let yend = y1 + gradient * (xend - x1);
        let xgap = 1. - (xend - x1);
        let xpxl2 = xend;
        let ypxl2 = yend.floor();
        let yend_fract = yend - yend.floor();
        if steep {
            // grid[[ypxl2 as usize, xpxl2 as usize]] = (1. - yend.fract()) * xgap;
            // grid[[ypxl2 as usize + 1, xpxl2 as usize]] = yend.fract() * xgap;
            // trace!("draw_line 2 steep");
            // trace!("xgap: {}", xgap);
            // trace!("yend.fract(): {}", yend.fract());
            Self::max_pixel_on_grid(grid, ypxl2 as i8, xpxl2 as i8, (1. - yend_fract) * xgap);
            Self::max_pixel_on_grid(grid, ypxl2 as i8 + 1, xpxl2 as i8, yend_fract * xgap);
        } else {
            // grid[[xpxl2 as usize, ypxl2 as usize]] = (1. - yend.fract()) * xgap;
            // grid[[xpxl2 as usize, ypxl2 as usize + 1]] = yend.fract() * xgap;
            // trace!("draw_line 2 !steep");
            // trace!("xgap: {}", xgap);
            // trace!("yend.fract(): {}", yend.fract());
            Self::max_pixel_on_grid(grid, xpxl2 as i8, ypxl2 as i8, (1. - yend_fract) * xgap);
            Self::max_pixel_on_grid(grid, xpxl2 as i8, ypxl2 as i8 + 1, yend_fract * xgap);
        }

        if steep {
            for x in (xpxl1 as i8 + 1)..=(xpxl2 as i8 - 1) {
                // grid[[intery.floor() as usize, x]] = (1. - intery.fract());
                // grid[[intery.floor() as usize + 1, x]] = intery.fract();
                // trace!("draw_line l steep");
                // trace!("intery.fract(): {}", intery.fract());
                let intery_fract = intery - intery.floor();
                Self::max_pixel_on_grid(grid, intery.floor() as i8, x, 1. - intery_fract);
                Self::max_pixel_on_grid(grid, intery.floor() as i8 + 1, x, intery_fract);
                intery += gradient;
            }
        } else {
            for x in (xpxl1 as i8 + 1)..=(xpxl2 as i8 - 1) {
                // grid[[x, intery.floor() as usize]] = (1. - intery.fract());
                // grid[[x, intery.floor() as usize + 1]] = intery.fract();
                // trace!("draw_line l !steep");
                // trace!("intery.fract(): {}", intery.fract());
                let intery_fract = intery - intery.floor();
                Self::max_pixel_on_grid(grid, x, intery.floor() as i8, 1. - intery_fract);
                Self::max_pixel_on_grid(grid, x, intery.floor() as i8 + 1, intery_fract);
                intery += gradient;
            }
        }
    }

    fn get_cube_pixel_grid(&mut self) -> Array2<f64> {
        let mut grid = Array2::<f64>::zeros((9, 9)).to_owned();

        let points = self.get_cube_points();

        let a = points.row(0);
        let b = points.row(1);
        let c = points.row(2);
        let d = points.row(3);
        let e = points.row(4);
        let f = points.row(5);
        let g = points.row(6);
        let h = points.row(7);

        Self::draw_line(
            &mut grid,
            &a.slice(s![0..2]).to_owned(),
            &b.slice(s![0..2]).to_owned(),
        );
        Self::draw_line(
            &mut grid,
            &a.slice(s![0..2]).to_owned(),
            &c.slice(s![0..2]).to_owned(),
        );
        Self::draw_line(
            &mut grid,
            &a.slice(s![0..2]).to_owned(),
            &e.slice(s![0..2]).to_owned(),
        );

        Self::draw_line(
            &mut grid,
            &b.slice(s![0..2]).to_owned(),
            &d.slice(s![0..2]).to_owned(),
        );
        Self::draw_line(
            &mut grid,
            &b.slice(s![0..2]).to_owned(),
            &f.slice(s![0..2]).to_owned(),
        );

        Self::draw_line(
            &mut grid,
            &c.slice(s![0..2]).to_owned(),
            &d.slice(s![0..2]).to_owned(),
        );
        Self::draw_line(
            &mut grid,
            &c.slice(s![0..2]).to_owned(),
            &g.slice(s![0..2]).to_owned(),
        );

        Self::draw_line(
            &mut grid,
            &d.slice(s![0..2]).to_owned(),
            &h.slice(s![0..2]).to_owned(),
        );

        Self::draw_line(
            &mut grid,
            &e.slice(s![0..2]).to_owned(),
            &f.slice(s![0..2]).to_owned(),
        );
        Self::draw_line(
            &mut grid,
            &e.slice(s![0..2]).to_owned(),
            &g.slice(s![0..2]).to_owned(),
        );

        Self::draw_line(
            &mut grid,
            &f.slice(s![0..2]).to_owned(),
            &h.slice(s![0..2]).to_owned(),
        );

        Self::draw_line(
            &mut grid,
            &g.slice(s![0..2]).to_owned(),
            &h.slice(s![0..2]).to_owned(),
        );

        grid
    }
}
