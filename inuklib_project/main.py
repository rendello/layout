#!/opt/local/bin/python3.11

from typing import Dict, List

from datetime import datetime
from os import chdir, mkdir
from pathlib import Path
from shutil import copy, copytree, rmtree
from subprocess import run
from sys import exit, stderr
from tempfile import TemporaryDirectory
from zoneinfo import ZoneInfo


def build_print(s: str, is_error=False) -> str:
	time = datetime.now(ZoneInfo("Canada/Eastern")).astimezone().strftime("%H:%M")
	if not is_error:
		print(f"Inuklib {time}\t{s}")
	else:
		print(f"Inuklib [error] {time}\t{s}", file=stderr)
		exit(1)

def ensure_binaries_installed():
	not_installed_binaries = []
	for b in ["wasm-opt", "wasm-bindgen", "cargo"]:
		try:
			run([b, "--version"], check=True, capture_output=True)
		except FileNotFoundError:
			not_installed_binaries.append(b)

	if not_installed_binaries != []:
		build_print(f"Not installed: {not_installed_binaries}", is_error=True)

def build_wasm_artifacts(artifact_dir: Path):
	mkdir(artifact_dir)

	build_print("Building WASM")
	run(["cargo", "build",
		"--release",
		"--lib",
		"--features", "wasm",
		"--target", "wasm32-unknown-unknown",
	], check=True)

	wasm_out = str(artifact_dir.joinpath("inuklib.wasm"))

	copy("target/wasm32-unknown-unknown/release/inuklib.wasm", wasm_out)

	run(["wasm-bindgen", wasm_out,
		"--out-dir", str(artifact_dir),
		"--target", "web",
		"--no-typescript"
	], check=True)

	run(["wasm-opt", wasm_out,
		"--output", wasm_out,
		"-O"
	], check=True)

def build_all():
	static_dir = Path("inuklib/static")

	with TemporaryDirectory(prefix="inuklib--") as temp_dir:
		temp_dir = Path(temp_dir)
		artifact_dir = temp_dir / "artifact"
		staging_dir = temp_dir / "dist"

		build_wasm_artifacts(artifact_dir)

		targets_and_static_paths = [
			("extension/firefox", ["common", "extension/common", "extension/firefox"]),
			("extension/chrome",  ["common", "extension/common", "extension/chrome"]),
			("web",               ["common", "web"]),
		]

		for relative_target_path, relative_static_paths in targets_and_static_paths:
			target_path = staging_dir / relative_target_path
			copytree(artifact_dir, target_path)

			for relative_static_path in relative_static_paths:
				copytree(static_dir / relative_static_path, target_path, dirs_exist_ok=True)

		rmtree("inuklib/dist")
		copytree(staging_dir, "inuklib/dist")


if __name__ == "__main__":
	ensure_binaries_installed()

	build_all()