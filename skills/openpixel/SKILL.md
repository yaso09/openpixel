---
name: openpixel-pixel-art
description: Create, refine, and animate readable pixel art in OpenPixel JSON, using its TypeScript renderer, Rust CLI, or MCP. Use when an agent needs to author an OpenPixel image or animation, improve pixel clusters, silhouette, palette, shading, or frame timing, or turn a visual brief into valid OpenPixel assets.
---

# OpenPixel Pixel Art Craft

Create deliberate, readable pixel art and animations that work in OpenPixel's actual format. Treat every pixel as a design decision. Prefer a strong shape and a small number of purposeful clusters over noisy detail or a raster image that only looks pixelated after downscaling.

## First: understand the deliverable

Extract the subject, action, mood, intended use, canvas size, transparency/background needs, palette constraints, and whether the output is a still or an animation. Preserve exact user constraints. If a critical choice is missing, use a sensible default and state it briefly; ask only when the missing choice changes the deliverable materially.

Use the exact requested canvas dimensions. Otherwise choose a compact canvas suited to the subject and its smallest necessary features; 16x16, 24x24, and 32x32 are useful starting points, not rules. Keep a one-pixel feature only when it survives inspection at native size. Leave transparent negative space when it helps the silhouette.

If the user supplies a reference, use it to understand subject, pose, mood, and broad color relationships. Create a fresh arrangement; do not trace or reproduce a living artist's distinctive style. Describe visual attributes instead (limited warm palette, chunky clusters, high contrast, top-left light).

## OpenPixel contract

Read the local schema and examples when present: `schemas/pixel-art.v1.schema.json` and `examples/`. The v1 document shape is:

```json
{
  "version": 1,
  "name": "Ember moth",
  "width": 16,
  "height": 16,
  "loop": true,
  "layers": [{ "id": "body", "name": "Body", "visible": true, "opacity": 1 }],
  "frames": [{
    "durationMs": 120,
    "layers": [{ "layerId": "body", "pixels": [[null, "#F2A65A"]] }]
  }]
}
```

The example matrix is abbreviated: every layer matrix must contain exactly `height` rows, each with exactly `width` entries. Use `null` for transparency and `#RRGGBB` (or schema-supported `#RRGGBBAA`) for pixels. Each frame contains one matrix for every declared layer, keyed by its exact `layerId`. Layer array order is back to front. IDs use letters, digits, `_`, or `-`. Frame durations are integer milliseconds from 1 through 60000. Coordinates and frame indexes are zero-based.

Keep transparent art transparent unless the user asks for a backdrop. Use a background layer only when the background is part of the artwork. Use separate layers for parts that need independent visibility or different animation, not for every shading color. The renderer uses crisp SVG edges; it does not blur or smooth pixels for you.

## Craft workflow

### 1. Plan the image at thumbnail scale

Write a one-line subject statement and identify the focal point. Decide on a clear silhouette, view angle, canvas footprint, and a simple foreground/midground/background plan if it is a scene. Reserve negative space around important forms. Check that the subject remains identifiable as a single dark shape before adding details.

For characters and creatures, establish head/body proportions, facing direction, limb placement, and a readable pose. For objects, establish the outer contour and major planes. For scenes, use a foreground anchor, a focal subject, and a quieter background; avoid giving every area equal contrast.

### 2. Block in large shapes

Build large flat masses first. Use a small ASCII/block map or coordinate notes if it helps keep geometry coherent, then translate it into matrices. Keep clusters connected and shaped: avoid stray single pixels, accidental holes, uneven stair steps, and long parallel pixel bands. Use deliberate 1-pixel accents only when they clarify a feature or sparkle.

Inspect contours at native size. On diagonals and curves, use intentional step lengths and consistent rhythm. Pixel art does not require anti-aliasing: leave a hard edge when that is clearer. Add a small intermediate-color ramp only to solve a specific edge or value transition; avoid automatic smoothing and muddy outlines.

### 3. Choose a functional palette

Start with a compact palette whose colors have jobs: outline/deep shadow, local midtone, light, and one or two accents. Choose colors by value separation first, then hue and saturation. Shift hue across a ramp where it supports the material or light (often cooler/darker shadows and warmer/brighter highlights), but do not apply this mechanically. Make the focal point carry the clearest contrast. Reuse colors across materials to keep the image unified.

Check the image in grayscale or as a solid silhouette: important regions should still separate. Avoid near-duplicate colors, indiscriminate gradients, and pure black outlines everywhere. A recognizable published palette can be a starting constraint, but never assume every palette fits every subject. Cite or identify a palette when you deliberately adopt one.

### 4. Shade and texture with restraint

Keep the light source consistent across planes and layers. Use a few readable shadow and highlight clusters that describe form. Prefer broad planes and selective highlights over a smooth ramp on every object. Use dithering only when a patterned texture or controlled transition benefits the image; keep it localized and purposeful. Do not let checkerboards replace form shading. Avoid pillow-shading (uniform bands around forms), noisy pixel scatter, excessive texture, and banding where similarly sized color steps form accidental stripes.

### 5. Refine by editing clusters

Zoom in to find broken contours, isolated pixels, accidental tangencies, inconsistent outline thickness, merged forms, and clusters that fight the intended light. Zoom back out to judge readability and focal hierarchy. Remove details that do not help recognition, material, depth, or mood. Pixel count and palette size are tools, not quality scores.

### 6. Build animation from poses

For motion, first define the action and its readable key poses (for example: anticipation, contact/action, recovery). Decide which layers stay still and which move. Make pose changes meaningful at the target resolution; do not merely flicker colors or shift the whole sprite unless that is the intended effect.

Preserve character volume, scale, pivot/foot contact, silhouette, light direction, and palette across frames. Use holds where the action needs emphasis and shorter durations for quick events; timing is a design choice, not a fixed FPS recipe. Start with a few frames, then add only frames that improve the motion. For loops, make the transition from the last frame back to the first feel intentional. Aseprite's concepts of timeline, layers, frame durations, onion skinning, and loop sections are useful references, but OpenPixel v1 currently stores ordered frames and one global `loop` flag; it has no animation tags or onion-skin metadata.

For each frame, check the full layer set and matrix dimensions. Inspect all frames in sequence, not just the first and last. Keep static layers byte-for-byte identical across frames when possible. Use frame durations to express timing; never assume all frames should have the same duration.

## Use OpenPixel tools deliberately

Discover what is available in the current environment before choosing an interface. Do not invent a tool, command, image preview, or write result.

- **MCP**: use `create_asset`, `set_pixel`, `set_layer`, `add_frame`, `set_frame_duration`, `set_loop`, `inspect_asset`, `validate_asset`, `render_frame`, and `render_all_frames` as appropriate. MCP edits take explicit source and output paths. `set_pixel` edits one coordinate; for larger generated images, author the JSON matrices directly when file access is available, then validate and render through MCP.
- **CLI**: use `openpixel new`, `set-pixel`, `set-layer`, `add-frame`, `set-frame`, `set-loop`, `inspect`, `validate`, `render`, and `render-frames`. Write edits to an explicit output path. Example: `openpixel render art.json --frame 0 --scale 8 --output preview.svg`; `openpixel render-frames art.json --scale 8 --out-dir previews`.
- **TypeScript renderer**: call `validatePixelArt(asset)` before `renderFrameToSvg(asset, { frameIndex, pixelSize, background, title })`. `mountPixelArt(element, asset)` is browser-only playback. Use the actual package API and existing build state.
- If MCP or CLI is unavailable, create a v1 JSON file only if workspace file editing is available. Say which checks could not be run.

The CLI exports SVGs, one per frame. OpenPixel does not currently encode a video file. If a video is requested, create and validate the asset, export the numbered SVG frames, then use an available external raster/video encoder only if the environment provides one; state any missing encoder or format conversion rather than claiming video output exists. SVG remains vector containers of crisp pixel rectangles, so preserve the pixel grid when converting.

## Quality gate before delivery

1. Validate against the local JSON Schema and OpenPixel validator when available.
2. Confirm every frame has all declared layers and every matrix has exactly the requested dimensions.
3. Inspect at native size and at an integer enlargement with nearest/crisp edges.
4. Check silhouette, focal hierarchy, value contrast, palette reuse, light direction, and transparency.
5. Check contours for jaggies, broken clusters, accidental tangencies, stray pixels, banding, pillow-shading, and noisy dithering.
6. For animation, inspect every frame and timing, confirm volume/anchor consistency, and preview the loop boundary.
7. Render the requested frame or all frames. If the UI supports previews, look at the rendered result; an SVG being generated successfully is not visual proof of craft quality.
8. Report output paths, dimensions, frame count/durations, palette size when known, and validation/render results. Mention limitations plainly.

Do not add a palette restriction, background, animation loop, or extra ornament that the user did not request. Do not equate technical validity with artistic quality.

## Prompt plan for image-capable models

When another image-generation model is available, give it concrete production constraints, then translate or clean the result into true grid-aligned OpenPixel matrices. A useful brief names: subject and action; exact canvas; viewpoint and silhouette; focal hierarchy; a restrained palette with light direction; cluster-based shading; transparency; and explicit exclusions (no gradients, blur, anti-aliased edges, subpixel placement, text, or extra objects unless wanted). Image generation is a draft source, not proof of a valid OpenPixel asset. Inspect at native resolution, rebuild or clean clusters, map colors to the chosen palette, and run the quality gate.

## Reliable references

Use these as technique references, not as mandatory style rules:

- Cure, [The Pixel Art Tutorial](https://pixeljoint.com/forum/forum_posts.asp?TID=11299) — clusters, jaggies, anti-aliasing, dithering, banding, pillow-shading, noise, and palette practice.
- Lospec, [Pixel Art Palette Library](https://lospec.com/palette-list) and [Developer API documentation](https://api.lospec.com/docs/) — palette discovery and color-count filtering. Treat community palettes as authored resources; credit a selected palette where appropriate.
- Aseprite, [Sprite basics](https://www.aseprite.org/docs/basics/), [Animation](https://aseprite.com/docs/animation/), and [Sprite structure](https://www.aseprite.org/docs/sprite/) — frames, layers, cels, durations, preview, and loop workflows.
- Saultoons, [Pixel Art Animation Tutorial](https://www.youtube.com/watch?v=iWvfaiiVuDI) — reference breakdown, key poses, smear frames, timing, and secondary motion.
- Toronja GB, [Anti-aliasing: The Swiss Army Knife in Pixel Art](https://www.youtube.com/watch?v=Bm1gRlXWUT4) — AA, banding, pillow-shading, and when smoothing is unnecessary.

Source pages were checked on 2026-10-29. Recheck current project schema and tool docs before relying on implementation details; technique advice is a set of choices, not universal law.
