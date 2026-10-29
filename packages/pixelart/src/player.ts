import { renderFrameToSvg } from "./render.js";
import { validatePixelArt } from "./validate.js";
import type { PixelArtDocument, PixelArtPlayer, PlayerOptions } from "./types.js";

/** Mounts a controllable, timer-driven animation in a browser element. */
export function mountPixelArt(target: Element, document: PixelArtDocument, options: PlayerOptions = {}): PixelArtPlayer {
  validatePixelArt(document);
  let index = 0;
  let timer: ReturnType<typeof setTimeout> | undefined;
  let disposed = false;
  const draw = () => {
    target.innerHTML = renderFrameToSvg(document, { ...options, frameIndex: index });
    options.onFrame?.(index);
  };
  const clearTimer = () => { if (timer !== undefined) clearTimeout(timer); timer = undefined; };
  const schedule = () => {
    clearTimer();
    if (disposed || document.frames.length < 2) return;
    timer = setTimeout(() => {
      if (index + 1 < document.frames.length) index += 1;
      else if (document.loop !== false) index = 0;
      else return;
      draw();
      schedule();
    }, document.frames[index].durationMs);
  };
  draw();
  if (options.autoplay !== false) schedule();
  return {
    get frameIndex() { return index; },
    play() { if (!disposed) schedule(); },
    pause() { clearTimer(); },
    seek(frameIndex: number) {
      if (!Number.isInteger(frameIndex) || frameIndex < 0 || frameIndex >= document.frames.length) throw new RangeError("frameIndex is outside the animation");
      index = frameIndex; draw(); if (timer !== undefined) schedule();
    },
    dispose() { disposed = true; clearTimer(); }
  };
}
