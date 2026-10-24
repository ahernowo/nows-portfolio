"""Render site/static/images/og.png (1200x630), the card shown when a page is shared.

Run from the repository folder after changing the name, role or colours:

    pip install pillow fonttools brotli
    python scripts/make-og-image.py

It reads the site's own font, Inter, from site/static/fonts (converting the woff2
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


INTER = to_ttf("inter-latin-opsz-normal.woff2", "inter.ttf")

W, H = 1200, 630
PAPER, INK, SOFT, RULE = "#FAF5F3", "#2B1618", "#6B5557", "#E1DAD9"
PEACH, ROSE, STAMP = "#F9E3DB", "#F6E2E6", "#B10F2E"
SLIP = "#FFFFFF"
S = 2  # supersample


def font(path, size, weight, opsz=32):
    """Inter at a weight (100-900) and optical size (14-32)."""
    f = ImageFont.truetype(path, size * S)
    # Inter has two axes, opsz and wght. Pillow wants their values in the
    # font's own axis order, so read that order instead of assuming it.
    values = []
    for axis in f.get_variation_axes():
        label = str(axis["name"]).lower()
        values.append(weight if "weight" in label else opsz)
    f.set_variation_by_axes(values)
    return f


def fitted(path, text, max_width, size, weight, opsz=32):
    """The largest size up to `size` at which `text` fits in `max_width` px."""
    while size > 10:
        f = font(path, size, weight, opsz)
        if f.getlength(text) <= max_width * S:
            return f
        size -= 1
    return font(path, size, weight, opsz)

img = Image.new("RGB", (W * S, H * S), PAPER)
d = ImageDraw.Draw(img)

# The slips start at x=690, so every line on the left stops by x=660.
name = font(INTER, 168, 700)
role = fitted(INTER, "Digital systems for fundraising", 660 - 90, 40, 500)
tag = fitted(INTER, "Donation forms, payment gateways,", 660 - 90, 30, 400, 24)
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
    f1 = font(INTER, 20, 650, 20); f2 = font(INTER, 17, 400, 17); f3 = font(INTER, 17, 600, 17)
    ld.text((18 * S, 16 * S), label, font=f1, fill=INK)
    yy = 52
    for k, v in lines:
        ld.line([(18 * S, yy * S), ((sw - 18) * S, yy * S)], fill=RULE, width=S)
        ld.text((18 * S, (yy + 8) * S), k, font=f2, fill=SOFT)
        tw = ld.textlength(v, font=f3)
        ld.text(((sw - 18) * S - tw, (yy + 8) * S), v, font=f3, fill=INK)
        yy += 36
    shadow = Image.new("RGBA", layer.size, (0, 0, 0, 0))
    ImageDraw.Draw(shadow).rounded_rectangle([0, 0, sw * S - 1, sh * S - 1], radius=4 * S, fill=(43, 22, 24, 70))
    shadow = shadow.rotate(angle, expand=True, resample=Image.BICUBIC).filter(ImageFilter.GaussianBlur(12 * S))
    img.paste(shadow, ((x + 4) * S, (y + 14) * S), shadow)
    rot = layer.rotate(angle, expand=True, resample=Image.BICUBIC)
    img.paste(rot, (x * S, y * S), rot)

slip(SLIP, "Donation form", [("Reference", "DN-2041"), ("Amount", "Rp150.000"), ("Plan", "Monthly, card")], 2.5, 690, 42)
slip(PEACH, "Payment gateway", [("Reference", "DN-2041"), ("Settled", "Rp150.000"), ("Status", "Captured")], -1.75, 735, 196)
slip(ROSE, "CRM record", [("Reference", "DN-2041"), ("Donor", "Recurring"), ("Gift", "Rp150.000")], 1, 780, 350)

# The stamp, with uneven ink.
st = Image.new("RGBA", (300 * S, 110 * S), (0, 0, 0, 0))
sd = ImageDraw.Draw(st)
# Inter runs wider than the old narrow face; keep the word inside the frame.
sf = fitted(INTER, "Reconciled", 250, 54, 800)
sd.rounded_rectangle([10 * S, 14 * S, 290 * S, 96 * S], radius=8 * S, outline=STAMP, width=5 * S)
sd.rounded_rectangle([3 * S, 7 * S, 297 * S, 103 * S], radius=10 * S, outline=STAMP, width=2 * S)
tw = sd.textlength("Reconciled", font=sf)
top, bottom = sd.textbbox((0, 0), "Reconciled", font=sf)[1::2]
sd.text(((300 * S - tw) / 2, 55 * S - (top + bottom) / 2), "Reconciled", font=sf, fill=STAMP)
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
