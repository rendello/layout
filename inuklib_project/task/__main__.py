#!/opt/local/bin/python3.11

import argparse
import os
import pathlib
from pathlib import Path
from subprocess import run

import build_wasm

from common import build_print, build_exit, PROJECT_ROOT, ASSET_PATH
from generate import generate


def ensure_binaries_installed():
    not_installed_binaries = []
    for b in ["wasm-opt", "wasm-bindgen", "cargo", "cargo-about"]:
        try:
            run([b, "--version"], check=True, capture_output=True)
        except FileNotFoundError:
            not_installed_binaries.append(b)

    if not_installed_binaries != []:
        build_exit(f"Not installed: {not_installed_binaries}")


def update_data_file(assume_yes: bool):
    data_file_path = PROJECT_ROOT / "inuklib/src/data.rs"

    generated = generate(ASSET_PATH / "table.tsv", ASSET_PATH / "wordlist.txt")

    with open(data_file_path, 'r') as file:
        if generated == file.read():
            build_print(f"`{data_file_path}` is identical to generated code, not updating.")
            return

    if not assume_yes:
        if input(f"Overwrite `{data_file_path}`? [Y/n]: ").upper() != "Y":
            build_exit("Aborting.")

    with open(data_file_path, "w") as file:
        file.write(generated)
        build_print(f"`{data_file_path}` updated")


# ----------

BUILD_WASM_HELP = \
    "build webpage and web extensions"

NO_LICENCE_HELP = \
    "build without generating acknowledgements webpage (which requires Internet)"

OPT_HELP = \
    "level of WASM optimization. `-O` is default, `-O0` is none; see `wasm-opt -h`"

parser = argparse.ArgumentParser(prog="inuklib")
subparsers = parser.add_subparsers(dest="command")

parser_build_wasm = subparsers.add_parser("build-wasm", help=BUILD_WASM_HELP)
parser_build_wasm.add_argument("--no-license", action="store_true", help=NO_LICENCE_HELP)
parser_build_wasm.add_argument('-O', nargs="?", choices="01234sz", help=OPT_HELP)

parser_generate_data = subparsers.add_parser("generate-data", help="regenerate `data.rs`")
parser_generate_data.add_argument("-y", "--assume-yes", action="store_true", help="no prompt on overwrite")

# ----------

if __name__ == "__main__":
    ensure_binaries_installed()

    args = parser.parse_args()

    match args.command:
        case "build-wasm":
            build_wasm.build_all(PROJECT_ROOT, not args.no_license, opt_level=args.O)
        case "generate-data":
            update_data_file(args.assume_yes)