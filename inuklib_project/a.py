#!/opt/local/bin/python3.11

import argparse

parser = argparse.ArgumentParser(
	prog="inuklib"
	)

subparsers = parser.add_subparsers()

parser_build = subparsers.add_parser("build")
parser_build.add_argument("targets", nargs='+', choices=["wasm"],
                    help="Build targets.")


args = parser.parse_args()

print(args)