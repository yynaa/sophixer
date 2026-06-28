import { pointInRectCoords } from "../utils";
import { Widget } from "../widget";

export class Knob extends Widget {
  x: number;
  y: number;
  w: number;
  h: number;
  sens: number;
  label: string;
  fillStyle: string;

  lastX: number = 0;
  lastY: number = 0;
  touchIdentifier: number | null = null;

  value: number = 0;

  constructor(
    x: number,
    y: number,
    w: number,
    h: number,
    label: string,
    sens: number,
    fillStyle: string,
  ) {
    super();
    this.x = x;
    this.y = y;
    this.w = w;
    this.h = h;
    this.label = label;
    this.sens = sens;
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

    ctx.fillStyle = this.fillStyle;
    ctx.fillRect(coords[0], coords[1], coords[2] * this.value, coords[3]);
  }

  public override onTouchStart(event: TouchEvent): void {
    for (let i = 0; i < event.touches.length; i++) {
      const touch = event.touches.item(i);
      if (
        touch !== null &&
        pointInRectCoords(touch.clientX, touch.clientY, this.getCoordinates())
      ) {
        this.touchIdentifier = touch.identifier;
        this.lastX = touch.clientX;
        this.lastY = touch.clientY;
      }
    }
  }

  public override onTouchMove(event: TouchEvent): void {
    for (let i = 0; i < event.touches.length; i++) {
      const touch = event.touches.item(i);

      if (touch !== null && touch.identifier === this.touchIdentifier) {
        this.value +=
          (touch.clientX - this.lastX) * this.sens -
          (touch.clientY - this.lastY) * this.sens;
        this.value = Math.max(0, Math.min(1, this.value));

        this.lastX = touch.clientX;
        this.lastY = touch.clientY;
      }
    }
  }

  public override onTouchEnd(event: TouchEvent): void {
    this.touchIdentifier = null;
  }
}
