#!/opt/local/bin/python3.11

import argparse
from subprocess import run
from dataclasses import dataclass
from pathlib import Path

import build_wasm

from common import build_print, build_exit, PROJECT_ROOT, ASSET_PATH
from generate import generate
from generate_utf8 import generate_utf8_case_data


def get_data_file_path(file_name: str) -> Path:
    path = PROJECT_ROOT / "inuklib/src/data" / file_name
    assert path.is_file()
    return path

def ensure_binaries_installed():
    not_installed_binaries = []
    for b in ["wasm-opt", "wasm-bindgen", "cargo", "cargo-about"]:
        try:
            run([b, "--version"], check=True, capture_output=True)
        except FileNotFoundError:
            not_installed_binaries.append(b)

    if not_installed_binaries != []:
        build_exit(f"Not installed: {not_installed_binaries}")


def update_data_file(data_file_path: Path, new_contents: str, assume_yes_bool: bool):
    with open(data_file_path, 'r') as file:
        if new_contents == file.read():
            build_print(f"`{data_file_path}` is identical to new version, not updating.")
            return

    if not assume_yes_bool:
        if input(f"Overwrite `{data_file_path}`? [Y/n]: ").upper() != "Y":
            build_exit("Aborting.")

    with open(data_file_path, "w") as file:
        file.write(new_contents)
        build_print(f"`{data_file_path}` updated")


def update_lookups_data_file(assume_yes_bool: bool):
    data_file_path = get_data_file_path("lookups.rs")
    generated = generate(ASSET_PATH / "table.tsv", ASSET_PATH / "wordlist.txt")
    update_data_file(data_file_path, generated, assume_yes_bool)


def update_utf8_case_data_file(assume_yes_bool: bool):
    data_file_path = get_data_file_path("utf8_case_data.rs")
    generated = generate_utf8_case_data()
    update_data_file(data_file_path, generated, assume_yes_bool)

# ----------

BUILD_WASM_HELP = \
    "build webpage and web extensions"

NO_LICENCE_HELP = \
    "build without generating acknowledgements webpage (which requires Internet)"

OPT_HELP = \
    "level of WASM optimization. `-O` is default, `-O0` is none; see `wasm-opt -h`"

COPY_ASSETS_HELP = \
    "copy static assets to a web target without having to recompile WASM artifacts"

parser = argparse.ArgumentParser(prog="inuklib")
subparsers = parser.add_subparsers(dest="command")

parser_build_wasm = subparsers.add_parser("build-wasm", help=BUILD_WASM_HELP)
parser_build_wasm.add_argument("--no-license", action="store_true", help=NO_LICENCE_HELP)
parser_build_wasm.add_argument("-O", nargs="?", choices="01234sz", help=OPT_HELP)

parser_generate_data = subparsers.add_parser("generate-lookups", help="regenerate `data/lookups.rs`")
parser_generate_data.add_argument("-y", "--assume-yes", action="store_true", help="no prompt on overwrite")

parser_generate_data = subparsers.add_parser("generate-utf8-case-data", help="regenerate `data/utf8_case_data.rs`")
parser_generate_data.add_argument("-y", "--assume-yes", action="store_true", help="no prompt on overwrite")

parser_copy_assets = subparsers.add_parser("copy-assets", help=COPY_ASSETS_HELP)
parser_copy_assets.add_argument("target", choices=["all"], help=OPT_HELP)

# ----------

if __name__ == "__main__":
    ensure_binaries_installed()

    args = parser.parse_args()

    match args.command:
        case "build-wasm":
            build_wasm.build_all(PROJECT_ROOT, not args.no_license, opt_level=args.O)
        case "generate-lookups":
            update_lookups_data_file(args.assume_yes)
        case "generate-utf8-case-data":
            update_utf8_case_data_file(args.assume_yes)
        case "copy-assets":
            build_wasm.copy_static_assets_all(PROJECT_ROOT)