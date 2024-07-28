#!/usr/bin/python3

import sh
from glob import glob


def build_wasm(out_dir):
	sh.cargo.build(
		"--lib",
		"--features", "wasm",
		"--target", "wasm32-unknown-unknown",
		"--release",
		_fg=True
	)

	sh.wasm_bindgen(
		"target/wasm32-unknown-unknown/release/analyzer.wasm",
		"--out-dir", out_dir,
		"--target", "web",
		"--no-typescript",
	)

	sh.wasm_opt(out_dir+"/analyzer_bg.wasm", "-o", out_dir+"/analyzer_bg.wasm-opt.wasm", "-O")
	sh.mv(out_dir+"/analyzer_bg.wasm-opt.wasm", out_dir+"/analyzer_bg.wasm")


def build_web():
	with TemporaryDirectory() as temp_dir:
		build_wasm(out_dir=temp_dir)
		sh.cp(temp_dir, "dist/web")