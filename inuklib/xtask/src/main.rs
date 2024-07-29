use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;
use wasm_bindgen_cli_support::Bindgen;
use wasm_opt::OptimizationOptions;
use cargo_metadata::MetadataCommand;
use anyhow::Result;


fn copy_dir_all(from_dir: impl AsRef<Path>, to_dir: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(&to_dir)?;
    for entry in fs::read_dir(from_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            copy_dir_all(entry.path(), to_dir.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), to_dir.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}


fn project_root() -> PathBuf {
    let root = MetadataCommand::new()
        .exec()
        .expect("Failed to obtain cargo metadata")
        .workspace_root;

    PathBuf::from(root)
}


fn static_path(s: &str) -> PathBuf {
    project_root().join("inuklib/static/").join(s)
}


fn target_path(s: &str) -> PathBuf {
    project_root().join("inuklib/dist/").join(s)
}


fn build_wasm_artifacts(out_dir: &Path) {
    Command::new("cargo")
        .current_dir(".")
        .args([
            "build",
            "--lib",
            "--features", "wasm",
            "--target", "wasm32-unknown-unknown",
            "--release"
        ])
        .output()
        .unwrap();

    fs::copy(
        project_root().join("target/wasm32-unknown-unknown/release/inuklib.wasm"),
        out_dir.join("inuklib.wasm")
    ).expect("");

    println!("{}", "Generating WASM bindings");
    Bindgen::new()
        .input_path(out_dir.join("inuklib.wasm"))
        .web(true)
        .expect("???")
        .generate(out_dir)
        .expect("Failed to run wasm-bindgen");

    let bind_gen_wasm = out_dir.join("inuklib_bg.wasm");

    println!("{}", "Optimizing WASM");
    OptimizationOptions::new_optimize_for_size()
        .run(&bind_gen_wasm, &bind_gen_wasm)
        .expect("opatronus");
}


fn build_all(clean: bool) -> Result<()> {
    if clean {
        fs::remove_dir_all(project_root().join("inuklib/dist"))?;
    }

    let artifact_dir = TempDir::with_prefix("inuklib-").expect("");
    build_wasm_artifacts(artifact_dir.path());

    let firefox_dirs = ["common", "extension/common", "extension/firefox"];
    let chrome_dirs  = ["common", "extension/common", "extension/chrome"];
    let web_dirs     = ["common", "web"];

    let targets_and_static_rel_paths: [(&str, &[&str]); 3] = [
        ("extension/firefox", &firefox_dirs),
        ("extension/chrome", &chrome_dirs),
        ("web", &web_dirs)
    ];

    for (target_rel_path, static_rel_paths) in targets_and_static_rel_paths {
        copy_dir_all(artifact_dir.path(), &target_path(target_rel_path)).expect("");
        for static_rel_path in static_rel_paths {
            copy_dir_all(static_path(static_rel_path), target_path(target_rel_path)).expect("");
        }
    }
    Ok(())
}


fn main() -> Result<()> {
    build_all(false)
}
