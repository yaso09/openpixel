# @openpixel/renderer

TypeScript renderer and browser animation player for OpenPixel v1 JSON assets. It has no runtime dependencies.

## Install

```sh
npm install @openpixel/renderer
```

## Render an SVG frame

```ts
import { renderFrameToSvg, type PixelArtDocument } from "@openpixel/renderer";

const asset: PixelArtDocument = {
  version: 1,
  name: "Spark",
  width: 2,
  height: 1,
  layers: [{ id: "base", name: "Base" }],
  frames: [{
    durationMs: 120,
    layers: [{ layerId: "base", pixels: [["#FFD166", "#FF6B6B"]] }]
  }]
};

const svg = renderFrameToSvg(asset, {
  frameIndex: 0,
  pixelSize: 12,
  background: "#202030",
  title: "Spark"
});
document.querySelector("#preview")!.innerHTML = svg;
```

## Play an animation

```ts
import { mountPixelArt } from "@openpixel/renderer";

const player = mountPixelArt(document.querySelector("#preview")!, asset, {
  autoplay: false,
  onFrame: (frameIndex) => console.log("Frame", frameIndex)
});

player.seek(1);
player.play();
player.pause();
player.dispose();
```

`renderToSvg(document, options)` is an alias for `renderFrameToSvg`. `validatePixelArt(document)` validates a runtime value and narrows its TypeScript type. Invalid documents throw `PixelArtValidationError`, which includes the `issues` list.

## JSON v1 format

An asset contains `version`, `width`, `height`, ordered `layers`, and one or more `frames`. Each frame has a `durationMs` and a full-size pixel matrix for every layer. `null` is transparent; pixels use `#RRGGBB` or `#RRGGBBAA`. Layers draw in array order, from back to front. `loop` defaults to `true`.

See [the shared JSON Schema](./schemas/pixel-art.v1.schema.json) and [example animation](https://github.com/yaso09/openpixel/blob/master/examples/two-frame-spark.json).

## API

- `validatePixelArt(value)` and `PixelArtValidationError`
- `renderFrameToSvg(document, options)` and `renderToSvg(document, options)`
- `mountPixelArt(element, document, options)` returning `PixelArtPlayer`
- Types: `PixelArtDocument`, `PixelLayer`, `PixelFrame`, `FrameLayer`, `Pixel`, `PixelMatrix`, `RenderOptions`, `PlayerOptions`

## License

MIT.
