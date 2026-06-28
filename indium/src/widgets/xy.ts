import { pointInRectCoords } from "../utils";
import { Widget } from "../widget";

export class XYPad extends Widget {
  x: number;
  y: number;
  w: number;
  h: number;
  label: string;
  fillStyle: string;

  touchIdentifier: number | null = null;

  vx: number = 0;
  vy: number = 0;

  constructor(
    x: number,
    y: number,
    w: number,
    h: number,
    label: string,
    fillStyle: string,
  ) {
    super();
    this.x = x;
    this.y = y;
    this.w = w;
    this.h = h;
    this.label = label;
    this.fillStyle = fillStyle;
  }

  private getCoordinates(): [number, number, number, number] {
    return [
      this.x * window.innerWidth,
      this.y * window.innerHeight,
      this.w * window.innerHeight,
      this.h * window.innerHeight,
    ];
  }

  public override render(ctx: CanvasRenderingContext2D): void {
    const coords = this.getCoordinates();

    ctx.fillStyle = "rgb(50 50 50)";
    ctx.fillRect(coords[0], coords[1], coords[2], coords[3]);

    const indicatorSize = 8;

    ctx.fillStyle = this.fillStyle;
    ctx.fillRect(
      coords[0] + coords[2] * this.vx - indicatorSize / 2,
      coords[1] + coords[3] * this.vy - indicatorSize / 2,
      indicatorSize,
      indicatorSize,
    );
  }

  public override onTouchStart(event: TouchEvent): void {
    for (let i = 0; i < event.touches.length; i++) {
      const touch = event.touches.item(i);
      if (
        touch !== null &&
        pointInRectCoords(touch.clientX, touch.clientY, this.getCoordinates())
      ) {
        this.touchIdentifier = touch.identifier;
      }
    }
  }

  public override onTouchMove(event: TouchEvent): void {
    const coords = this.getCoordinates();
    for (let i = 0; i < event.touches.length; i++) {
      const touch = event.touches.item(i);

      if (touch !== null && touch.identifier === this.touchIdentifier) {
        this.vx = (touch.clientX - coords[0]) / coords[2];
        this.vy = (touch.clientY - coords[1]) / coords[3];

        this.vx = Math.max(0, Math.min(1, this.vx));
        this.vy = Math.max(0, Math.min(1, this.vy));
      }
    }
  }

  public override onTouchEnd(event: TouchEvent): void {
    this.touchIdentifier = null;
  }
}
