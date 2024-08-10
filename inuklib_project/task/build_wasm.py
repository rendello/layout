from typing import Dict, List

from os import chdir, mkdir
from pathlib import Path
from shutil import copy, copytree, rmtree

from tempfile import TemporaryDirectory
from subprocess import run

from common import build_print, asset_path


def build_all(project_dir: Path, build_license_page: bool):
	""" Build Firefox and Chrome extensions and the website.

	Builds WASM artifacts from Rust project and binds them with JS.
	The WASM is then optimized. These artifacts are copied to the
	aforementioned targets' directories along with their assets from /static.
	"""

	static_dir = Path(project_dir, "static")

	with TemporaryDirectory(prefix="inuklib--") as temp_dir:
		temp_dir = Path(temp_dir)
		artifact_dir = temp_dir / "artifact"
		staging_dir = temp_dir / "dist"

		mkdir(artifact_dir)

		if build_license_page:
			build_print("Building license acknowledgements page")
			res = run(["cargo-about", "generate", str(asset_path("about.hbs"))
			], check=True, capture_output=True, text=True)

			with open(artifact_dir / "acknowledgements.html", "w") as ack_file:
				ack_file.write(res.stdout)
		else:
			build_print("Skipping building license acknowledgements page")

		build_print("Building WASM")
		run(["cargo", "build",
			"--release",
			"--lib",
			"--features", "wasm",
			"--target", "wasm32-unknown-unknown",
		], check=True)

		wasm_out = str(artifact_dir / "inuklib.wasm")

		copy("target/wasm32-unknown-unknown/release/inuklib.wasm", wasm_out)

		build_print("Generating bindings")
		run(["wasm-bindgen", wasm_out,
			"--out-dir", str(artifact_dir),
			"--target", "web",
			"--no-typescript"
		], check=True)

		build_print("Optimizing WASM")
		run(["wasm-opt", wasm_out,
			"--output", wasm_out,
			"-O"
		], check=True)

		build_print("Copying assets to staging directory")
		statics_and_targets = [
			(["common", "extension/common", "extension/firefox"], "extension/firefox", ),
			(["common", "extension/common", "extension/chrome"],  "extension/chrome"),
			(["common", "web"],                                   "web")
		]

		for relative_static_paths, relative_target_path in statics_and_targets:
			target_path = staging_dir / relative_target_path
			copytree(artifact_dir, target_path)

			for relative_static_path in relative_static_paths:
				static_path = static_dir / relative_static_path

				# Git doesn't track empty folders, so they may not exist.
				if static_path.is_dir():
					copytree(static_path, target_path, dirs_exist_ok=True)

		build_print("Copying staging directory to `dist/`")
		dist_path = Path(project_dir, "dist")
		if dist_path.is_dir():
			rmtree(dist_path)

		copytree(staging_dir, dist_path)