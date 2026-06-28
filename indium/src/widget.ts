import { canvasRoot } from "./dom";

export abstract class Widget {
  constructor() {
    canvasRoot.addEventListener("touchstart", (e) => this.onTouchStart(e));
    canvasRoot.addEventListener("touchend", (e) => this.onTouchEnd(e));
    canvasRoot.addEventListener("touchmove", (e) => this.onTouchMove(e));
  }

  public abstract render(ctx: CanvasRenderingContext2D): void;
  public abstract onTouchStart(event: TouchEvent): void;
  public abstract onTouchEnd(event: TouchEvent): void;
  public abstract onTouchMove(event: TouchEvent): void;
}

export let widgets: Widget[] = [];
