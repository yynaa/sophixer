export let canvasRoot = document.createElement("canvas");

export function initDOM() {
  canvasRoot.id = "rootCanvas";
  canvasRoot.addEventListener("touchend", () => {
    if (window.innerHeight !== screen.height) canvasRoot.requestFullscreen();
  });

  window.addEventListener("load", () => {
    document.body.appendChild(canvasRoot);

    canvasRoot.width = window.innerWidth;
    canvasRoot.height = window.innerHeight;

    window.addEventListener("resize", () => {
      canvasRoot.width = window.innerWidth;
      canvasRoot.height = window.innerHeight;
    });

    if (!canvasRoot.getContext) {
      const unsupported = document.createElement("p");
      unsupported.textContent = "this browser doesn't support canvases";
      document.body.appendChild(unsupported);
      document.body.removeChild(canvasRoot);
    }
  });
}
