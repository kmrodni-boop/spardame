#!/usr/bin/env python3
"""Rasterize the Spardame spade mark into PNG app icons."""

from pathlib import Path

from PIL import Image, ImageDraw

OUT = Path(__file__).resolve().parents[1] / "build" / "icons"
BG = (10, 36, 28, 255)
CREAM = (243, 234, 216, 255)
SIZES = (16, 24, 32, 48, 64, 128, 256, 512)


def rounded_mask(size: int, radius: int) -> Image.Image:
    mask = Image.new("L", (size, size), 0)
    d = ImageDraw.Draw(mask)
    d.rounded_rectangle((0, 0, size - 1, size - 1), radius=radius, fill=255)
    return mask


def draw_spade(draw: ImageDraw.ImageDraw, size: int) -> None:
    cx = size / 2
    # Tip sits a little above center; stem reaches the lower third.
    s = size * 0.34
    cy = size * 0.46
    r = s * 0.52
    # Lobes
    draw.ellipse((cx - s, cy - r * 0.15 - r, cx + 2, cy - r * 0.15 + r), fill=CREAM)
    draw.ellipse((cx - 2, cy - r * 0.15 - r, cx + s, cy - r * 0.15 + r), fill=CREAM)
    # Upper blade
    draw.polygon(
        [(cx, cy - s * 1.22), (cx - s * 1.02, cy + r * 0.22), (cx + s * 1.02, cy + r * 0.22)],
        fill=CREAM,
    )
    # Stem
    stem_top = cy + s * 0.18
    stem_bot = size * 0.86
    draw.polygon(
        [
            (cx - s * 0.10, stem_top),
            (cx + s * 0.10, stem_top),
            (cx + s * 0.42, stem_bot),
            (cx - s * 0.42, stem_bot),
        ],
        fill=CREAM,
    )


def render(size: int) -> Image.Image:
    img = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    layer = Image.new("RGBA", (size, size), BG)
    radius = max(2, round(size * 0.18))
    img.paste(layer, (0, 0), rounded_mask(size, radius))
    draw = ImageDraw.Draw(img)
    draw_spade(draw, size)
    return img


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    for size in SIZES:
        render(size).save(OUT / f"{size}x{size}.png")
    render(512).save(OUT / "icon.png")
    print(f"wrote {len(SIZES) + 1} icons in {OUT}")


if __name__ == "__main__":
    main()
