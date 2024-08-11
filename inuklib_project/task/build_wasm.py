from typing import Optional, List

from os import mkdir
from pathlib import Path
from shutil import copy, copytree, rmtree
from dataclasses import dataclass
from tempfile import TemporaryDirectory
from subprocess import run

from common import build_print, build_exit, ASSET_PATH


@dataclass(frozen=True)
class Target:
    relative_static_paths: List[str]
    relative_target_path: str

FIREFOX = Target(["common", "extension/common", "extension/firefox"], "extension/firefox")
CHROME =  Target(["common", "extension/common", "extension/chrome"],  "extension/chrome")
WEB =     Target(["common", "web"], "web")


def copy_static_assets(target: Target, static_base_dir: Path, target_base_dir: Path):
    target_path = target_base_dir / target.relative_target_path
    for relative_static_path in target.relative_static_paths:
        static_path = static_base_dir / relative_static_path

        # Git doesn't track empty folders, so they may not exist.
        if static_path.is_dir():
            copytree(static_path, target_path, dirs_exist_ok=True)


def copy_static_assets_all(project_dir):
    for target in [WEB, FIREFOX, CHROME]:
        copy_static_assets(target, project_dir / "static", project_dir / "dist")


def build_all(project_dir: Path, build_license_page: bool, opt_level: Optional[int]):
    """ Build Firefox and Chrome extensions and the website.

    Builds WASM artifacts from Rust project and binds them with JS.
    The WASM is then optimized. These artifacts are copied to the
    aforementioned targets' directories along with their assets from /static.
    """

    static_dir = Path(project_dir, "static")

    with TemporaryDirectory(prefix="inuklib--") as temp_dir_:
        temp_dir = Path(temp_dir_)
        artifact_dir = temp_dir / "artifact"
        staging_dir = temp_dir / "dist"

        mkdir(artifact_dir)

        if build_license_page:
            build_print("Building license acknowledgements page")
            res = run(["cargo-about", "generate", str(ASSET_PATH / "about.hbs")
            ], capture_output=True, text=True)

            if res.returncode != 0:
                build_exit(f"`cargo-about` failed with the following stderr:\n{res.stderr}")

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


        opt_flag = f"-O{opt_level if opt_level is not None else ''}"
        build_print(f"Optimizing WASM (level {opt_flag})")
        run(["wasm-opt", wasm_out,
            "--output", wasm_out,
            opt_flag
        ], check=True)

        for target in [WEB, FIREFOX, CHROME]:
            copytree(artifact_dir, staging_dir / target.relative_target_path)
            copy_static_assets(target, static_dir, staging_dir)

        build_print("Copying staging directory to `dist/`")
        dist_path = project_dir / "dist"
        if dist_path.is_dir():
            rmtree(dist_path)

        copytree(staging_dir, dist_path)