use ctru::prelude::*;

fn main() {
    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");
    let console = Console::new(gfx.top_screen.borrow_mut());
    console.select();

    println!("bismuth");

    while apt.main_loop() {
        hid.scan_input();
        let keys = hid.keys_down();

        if keys.contains(KeyPad::START) {
            break;
        }

        gfx.wait_for_vblank();
    }
}
