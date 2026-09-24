"""AdminDesk: generuje ikony i logo z admindesk/grafika/*.svg do miejsc, których używa RustDesk.

Uruchom z katalogu repo:  python admindesk/brand.py   (wymaga: pip install pillow resvg-py)
"""
import io
from pathlib import Path

import resvg_py
from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
SRC = ROOT / "admindesk" / "grafika"
TLO = (33, 40, 48, 255)  # granat z ikonka_logo.svg


def render(svg, width):
    png = resvg_py.svg_to_bytes(svg_path=str(SRC / svg), width=width)
    im = Image.open(io.BytesIO(bytes(png))).convert("RGBA")
    return im.crop(im.getbbox())


def recolor(im, rgb):
    out = Image.new("RGBA", im.size, rgb + (255,))
    out.putalpha(im.getchannel("A"))
    return out


def ikona(size=1024):
    """Białe „A” na zaokrąglonym granatowym kwadracie."""
    from PIL import ImageDraw

    im = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    ImageDraw.Draw(im).rounded_rectangle((0, 0, size - 1, size - 1), radius=size // 5, fill=TLO)
    znak = recolor(render("znak.svg", size), (255, 255, 255))
    znak.thumbnail((int(size * 0.68), int(size * 0.68)), Image.LANCZOS)
    im.alpha_composite(znak, ((size - znak.width) // 2, (size - znak.height) // 2))
    return im


def zapisz_ico(im, path, sizes):
    im.save(path, format="ICO", sizes=[(s, s) for s in sizes])


def logo(rgb, height=120):
    im = recolor(render("logo.svg", 4000), rgb)
    im.thumbnail((height * 10, height), Image.LANCZOS)
    return im


def main():
    ico = ikona()
    wszystkie = [16, 24, 32, 48, 64, 128, 256]
    zapisz_ico(ico, ROOT / "res/icon.ico", wszystkie)
    zapisz_ico(ico, ROOT / "res/tray-icon.ico", [16, 24, 32])
    zapisz_ico(ico, ROOT / "flutter/windows/runner/resources/app_icon.ico", wszystkie)
    zapisz_ico(ico, ROOT / "flutter/assets/icon.ico", wszystkie)
    for name, s in [("icon.png", 1024), ("mac-icon.png", 1024), ("32x32.png", 32),
                    ("64x64.png", 64), ("128x128.png", 128), ("128x128@2x.png", 256)]:
        ico.resize((s, s), Image.LANCZOS).save(ROOT / "res" / name)
    ico.resize((256, 256), Image.LANCZOS).save(ROOT / "flutter/assets/icon.png")
    # max 300x60 w UI (common.dart), dajemy 2x pod HiDPI
    logo((0, 0, 0)).save(ROOT / "flutter/assets/logo_light.png")
    logo((255, 255, 255)).save(ROOT / "flutter/assets/logo_dark.png")
    logo((0, 0, 0)).save(ROOT / "flutter/assets/logo.png")
    print("OK: ikony i logo wygenerowane")


if __name__ == "__main__":
    main()
