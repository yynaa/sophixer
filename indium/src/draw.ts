import { canvasRoot } from "./dom";
import { widgets } from "./widget";

function draw(ctx: CanvasRenderingContext2D) {
  ctx.fillStyle = "rgb(0 0 0)";
  ctx.fillRect(0, 0, window.innerWidth, window.innerHeight);

  for (const w of widgets) {
    w.render(ctx);
  }
}

export function initDraw() {
  window.addEventListener("load", () => {
    const ctx = canvasRoot.getContext("2d");
    if (ctx) {
      let t = 0;
      let lt: DOMHighResTimeStamp;

      const step = (timestamp: DOMHighResTimeStamp) => {
        if (lt === undefined) {
          lt = timestamp;
        }
        const dt = timestamp - lt;

        draw(ctx);

        t += dt;
        lt = timestamp;

        requestAnimationFrame(step);
      };

      requestAnimationFrame(step);
    }
  });
}
