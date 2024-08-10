#!/opt/local/bin/python3.11

import argparse
import os
import pathlib
from pathlib import Path
from subprocess import run

import build_wasm

from common import build_print, build_exit


PROJECT_ROOT_NAME = "inuklib_project"

def get_project_root():
	cwd = pathlib.Path.cwd()
	match _get_project_root(cwd.parts):
		case None:
			build_exit(
				f'''Project root, \"{PROJECT_ROOT_NAME}\" not '''
				f'''found in current working directory or ancestor: "{cwd}".''',
			)
		case parts:
			return Path(*parts).resolve()


def _get_project_root(parts):
	if parts == ():
		return None
	elif parts[-1] == PROJECT_ROOT_NAME:
		return parts
	else:
		return _get_project_root(parts[:-1])


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

parser = argparse.ArgumentParser(
	prog="inuklib"
	)

subparsers = parser.add_subparsers()

parser_build = subparsers.add_parser("build")
parser_build.add_argument("targets", nargs='+', choices=["wasm"],
                    help="Build targets.")

args = parser.parse_args()

print(args)

# ----------


if __name__ == "__main__":
	project_root = get_project_root()

	if project_root != pathlib.Path.cwd():
		build_print(f"Moving to {project_root}")
		os.chdir(project_root)

	ensure_binaries_installed()

	build_wasm.build_all(project_root)
