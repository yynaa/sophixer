import { initDOM } from "./dom";
import { initDraw } from "./draw";
import { widgets } from "./widget";
import { Knob } from "./widgets/knob";
import { XYPad } from "./widgets/xy";

initDOM();
initDraw();

widgets.push(new Knob(0, 0, 0.5, 0.2, "test", 0.004, "rgb(255 0 0)"));
widgets.push(new XYPad(0, 0.5, 0.5, 0.5, "test", "rgb(255 0 0)"));
