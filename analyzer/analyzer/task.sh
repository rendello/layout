#!/bin/bash -e


ensure_dist_structure() {
	mkdir -p dist/extension/firefox/
	mkdir -p dist/extension/chrome/
	mkdir -p dist/web/
}

# Generate WASM file, bind it with JS, and optimize it in a temporary directory.
# Echoes temporary directory path, which the caller must delete.
build_wasm() {
	wasm_filename=analyzer.wasm
	wasm_path="target/wasm32-unknown-unknown/release/$wasm_filename"
	wasm_bindgen_filename=analyzer_bg.wasm

	temp_dir=$(mktemp -d '/tmp/InuktitutAnalyzer.XXXXXXXXX')

	cargo build --lib --features wasm --target wasm32-unknown-unknown --release

	wasm-bindgen $wasm_path --out-dir $temp_dir --target web --no-typescript
	wasm-opt $temp_dir/$wasm_bindgen_filename -o $temp_dir/$wasm_bindgen_filename -O

	echo $temp_dir
	return 0
}

build_extension() {
	case $1 in
		firefox)
			build_extension_ firefox ;;
		chrome)
			build_extension_ chrome ;;
		all)
			build_extension_ firefox
			build_extension_ chrome
		;;
		*)
			echo "build_extension [firefox|chrome|all]"
			return 1
		;; 
	esac
	return 0
}

build_extension_() {
	target=$1

	wasm_artifacts_dir=$(build_wasm)

	rm -r dist/extension/$target/*
	cp -r \
		$wasm_artifacts_dir/*      \
		static/extension/$target/* \
		static/extension/common/*  \
		static/common/*            \
		dist/extension/$target/

	rm -r $wasm_artifacts_dir
	return 0
}

build_web() {
	wasm_artifacts_dir=$(build_wasm)

	rm -r dist/web/*
	cp -r \
		$wasm_artifacts_dir/* \
		static/web/*          \
		static/common/*       \
		dist/web/

	rm -r $wasm_artifacts_dir
	return 0
}


USAGE=$(cat <<-END
	task.sh build-extension [all|firefox|chrome]
	task.sh build-web
END
)



case $1 in
	build_extension)
		ensure_dist_structure
		build_extension $2
		;;
	build_web)
		ensure_dist_structure
		build_web
		;;
    *)
		echo -e "$USAGE"
		;;
esac