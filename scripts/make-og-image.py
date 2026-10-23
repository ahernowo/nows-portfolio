"""Render site/static/images/og.png (1200x630), the card shown when a page is shared.

Run from the repository folder after changing the name, role or colours:

    pip install pillow fonttools brotli
    python scripts/make-og-image.py

It reads the site's own fonts from site/static/fonts (converting the woff2
files to TrueType in a temporary folder, since Pillow can't read woff2).
"""
from pathlib import Path
import random
import tempfile

from fontTools.ttLib import TTFont
from PIL import Image, ImageDraw, ImageFont, ImageFilter

REPO = Path(__file__).resolve().parent.parent
FONTS = REPO / "site" / "static" / "fonts"
OUT = REPO / "site" / "static" / "images" / "og.png"
TMP = Path(tempfile.mkdtemp())


def to_ttf(woff2_name, ttf_name):
    font = TTFont(FONTS / woff2_name)
    font.flavor = None
    path = TMP / ttf_name
    font.save(path)
    return str(path)


ARCHIVO = to_ttf("archivo-latin-wdth-normal.woff2", "archivo.ttf")
NEWSREADER_ITALIC = to_ttf("newsreader-latin-opsz-italic.woff2", "newsreader-italic.ttf")

W, H = 1200, 630
PAPER, INK, SOFT, RULE = "#fcfcfa", "#1f2463", "#4a4f86", "#b9c9e6"
YELLOW, PINK, STAMP = "#fbf0b4", "#f9dce2", "#b4302a"
S = 2  # supersample

def font(path, size, axes):
    f = ImageFont.truetype(path, size * S)
    f.set_variation_by_axes(axes)
    return f

img = Image.new("RGB", (W * S, H * S), PAPER)
d = ImageDraw.Draw(img)

name = font(ARCHIVO, 168, [800, 68])
role = font(ARCHIVO, 40, [500, 100])
tag = font(NEWSREADER_ITALIC, 32, [400, 24])
d.text((84 * S, 150 * S), "Nows", font=name, fill=INK)
d.text((90 * S, 360 * S), "Digital systems for fundraising", font=role, fill=INK)
d.text((90 * S, 418 * S), "Donation forms, payment gateways,", font=tag, fill=SOFT)
d.text((90 * S, 458 * S), "reconciliation and reports.", font=tag, fill=SOFT)
d.line([(90 * S, 548 * S), (W * S - 90 * S, 548 * S)], fill=RULE, width=2 * S)

def slip(color, label, lines, angle, x, y):
    sw, sh = 330, 168
    layer = Image.new("RGBA", (sw * S, sh * S), (0, 0, 0, 0))
    ld = ImageDraw.Draw(layer)
    ld.rounded_rectangle([0, 0, sw * S - 1, sh * S - 1], radius=4 * S, fill=color)
    f1 = font(ARCHIVO, 20, [700, 85]); f2 = font(ARCHIVO, 17, [450, 100]); f3 = font(ARCHIVO, 17, [650, 100])
    ld.text((18 * S, 16 * S), label, font=f1, fill=INK)
    yy = 52
    for k, v in lines:
        ld.line([(18 * S, yy * S), ((sw - 18) * S, yy * S)], fill=RULE, width=S)
        ld.text((18 * S, (yy + 8) * S), k, font=f2, fill=SOFT)
        tw = ld.textlength(v, font=f3)
        ld.text(((sw - 18) * S - tw, (yy + 8) * S), v, font=f3, fill=INK)
        yy += 36
    shadow = Image.new("RGBA", layer.size, (0, 0, 0, 0))
    ImageDraw.Draw(shadow).rounded_rectangle([0, 0, sw * S - 1, sh * S - 1], radius=4 * S, fill=(31, 36, 99, 70))
    shadow = shadow.rotate(angle, expand=True, resample=Image.BICUBIC).filter(ImageFilter.GaussianBlur(12 * S))
    img.paste(shadow, ((x + 4) * S, (y + 14) * S), shadow)
    rot = layer.rotate(angle, expand=True, resample=Image.BICUBIC)
    img.paste(rot, (x * S, y * S), rot)

slip("#ffffff", "Donation form", [("Reference", "DN-2041"), ("Amount", "Rp150.000"), ("Plan", "Monthly, card")], 2.5, 690, 42)
slip(YELLOW, "Payment gateway", [("Reference", "DN-2041"), ("Settled", "Rp150.000"), ("Status", "Captured")], -1.75, 735, 196)
slip(PINK, "CRM record", [("Reference", "DN-2041"), ("Donor", "Recurring"), ("Gift", "Rp150.000")], 1, 780, 350)

# The stamp, with uneven ink.
st = Image.new("RGBA", (300 * S, 110 * S), (0, 0, 0, 0))
sd = ImageDraw.Draw(st)
sf = font(ARCHIVO, 54, [850, 68])
sd.rounded_rectangle([10 * S, 14 * S, 290 * S, 96 * S], radius=8 * S, outline=STAMP, width=5 * S)
sd.rounded_rectangle([3 * S, 7 * S, 297 * S, 103 * S], radius=10 * S, outline=STAMP, width=2 * S)
tw = sd.textlength("Reconciled", font=sf)
sd.text(((300 * S - tw) / 2, 22 * S), "Reconciled", font=sf, fill=STAMP)
random.seed(7)
px = st.load()
for _ in range(int(st.size[0] * st.size[1] * 0.12)):
    x, y = random.randrange(st.size[0]), random.randrange(st.size[1])
    r, g, b, a = px[x, y]
    if a:
        px[x, y] = (r, g, b, int(a * random.uniform(0.0, 0.5)))
st = st.rotate(9, expand=True, resample=Image.BICUBIC)
img.paste(st, (700 * S, 418 * S), st)

img = img.resize((W, H), Image.LANCZOS)
img.save(OUT, optimize=True)
print(f"Saved {OUT.relative_to(REPO)}")
