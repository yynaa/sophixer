export function pointInRect(
  px: number,
  py: number,
  bx: number,
  by: number,
  bw: number,
  bh: number,
): boolean {
  return px >= bx && px < bx + bw && py >= by && py < by + bh;
}

export function pointInRectCoords(
  px: number,
  py: number,
  coords: [number, number, number, number],
): boolean {
  return pointInRect(px, py, coords[0], coords[1], coords[2], coords[3]);
}
