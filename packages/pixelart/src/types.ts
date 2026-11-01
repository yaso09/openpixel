export type Pixel = string | null;
export type PixelMatrix = Pixel[][];

export interface PixelLayer {
  id: string;
  name: string;
  visible?: boolean;
  opacity?: number;
}

export interface FrameLayer {
  layerId: string;
  pixels: PixelMatrix;
}

export interface PixelFrame {
  durationMs: number;
  layers: FrameLayer[];
}

export interface PixelArtDocument {
  /** Artwork edit sequence; omitted legacy files are interpreted as revision 1. */
  revision?: number;
  version: 1;
  name?: string;
  width: number;
  height: number;
  loop?: boolean;
  layers: PixelLayer[];
  frames: PixelFrame[];
}

export interface RenderOptions {
  /** Integer scale factor represented by the SVG viewBox; artwork remains crisp. */
  pixelSize?: number;
  frameIndex?: number;
  /** Background color; omitted means transparent. */
  background?: string;
  title?: string;
}

export interface PlayerOptions extends Omit<RenderOptions, "frameIndex"> {
  autoplay?: boolean;
  onFrame?: (frameIndex: number) => void;
}

export interface PixelArtPlayer {
  readonly frameIndex: number;
  play(): void;
  pause(): void;
  seek(frameIndex: number): void;
  dispose(): void;
}
