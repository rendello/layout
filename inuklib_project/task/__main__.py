#!/opt/local/bin/python3.11

import argparse
import os
import pathlib
from pathlib import Path
from subprocess import run

import build_wasm

from common import build_print, build_exit, PROJECT_ROOT


def ensure_binaries_installed():
	not_installed_binaries = []
	for b in ["wasm-opt", "wasm-bindgen", "cargo", "cargo-about"]:
		try:
			run([b, "--version"], check=True, capture_output=True)
		except FileNotFoundError:
			not_installed_binaries.append(b)

	if not_installed_binaries != []:
		build_exit(f"Not installed: {not_installed_binaries}")

# ----------

BUILD_WASM_HELP = \
	"build webpage and web extensions"

NO_LICENCE_HELP = \
	"build without generating acknowledgements webpage (which requires Internet)"


parser = argparse.ArgumentParser(prog="inuklib")
subparsers = parser.add_subparsers(dest="command")



parser_build_wasm = subparsers.add_parser("build-wasm", help=BUILD_WASM_HELP)
parser_build_wasm.add_argument("--no-license", action="store_true", help=NO_LICENCE_HELP)

# ----------


if __name__ == "__main__":
	ensure_binaries_installed()

	args = parser.parse_args()

	if args.command == "build-wasm":
		build_wasm.build_all(PROJECT_ROOT, not args.no_license)