#!/usr/bin/env python3
"""1 = build & render (image.png), 2 = build & lint. Optional arg: phase dir."""
import subprocess, sys, pathlib

ROOT = pathlib.Path(__file__).parent
PHASE = ROOT / (sys.argv[2] if len(sys.argv) > 2 else "phase-2-rtnw")


def sh(cmd, **kw):
    print(f"$ {cmd if isinstance(cmd, str) else ' '.join(cmd)}")
    subprocess.run(cmd, cwd=PHASE, check=True, **kw)


def render():
    sh(["cargo", "build", "--release"])
    with open(PHASE / "image.ppm", "wb") as f:
        sh(["cargo", "run", "--release"], stdout=f)
    sh(["magick", "image.ppm", "image.png"])
    print(f"-> {PHASE / 'image.png'}")


def lint():
    sh(["cargo", "build"])
    sh(["cargo", "fmt", "--check"])
    sh(["cargo", "clippy", "--", "-D", "warnings"])


choice = sys.argv[1] if len(sys.argv) > 1 else input("1) render  2) lint\n> ").strip()
try:
    {"1": render, "2": lint}[choice]()
except KeyError:
    sys.exit("1 or 2")
except subprocess.CalledProcessError as e:
    sys.exit(e.returncode)
