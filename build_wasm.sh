#!/usr/bin/env bash

set -e

HELP_STRING=$(cat <<- END
	usage: build_wasm.sh PROJECT_NAME

	Build script for a Bevy project with wasm-bindgen.

	example: ./build_wasm.sh flappy-bird

	This'll go through the following steps:

	    1. Build as target 'wasm32-unknown-unknown'.
	    2. Create the directory 'dist' if it doesn't already exist.
	    3. Run wasm-bindgen with output into the 'dist' directory.
        4. Generate coresponding 'index.html' file.

	Required arguments:

	    PROJECT_NAME            The name of the artifact/target/project

	Author: Nik codes <nik.code.things@gmail.com>
	Version: 0.1
END
)

die () {
    echo >&2 "Error: $@"
    echo >&2
    echo >&2 "$HELP_STRING"
    exit 1
}

[ $# -ge 2 ] && die "too many arguments provided"
[ $# -le 0 ] && die "too few arguments provided"

PROJECT_NAME=$1

HTML=$(cat <<- END
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>${PROJECT_NAME}</title>
    <style>
        * {
        	padding: 0px;
        	margin: 0px;
    	}
    	canvas {
       		width: 100% !important;
        	height: 100% !important;
    	}
    </style>
</head>
<body>
    <script type="module">
  		import init from "./${PROJECT_NAME}.js";
  		init("./${PROJECT_NAME}_bg.wasm").then(function (wasm) {
    		wasm.run();
  		});
	</script>
</body>
</html>
END
)

# Delete dist
if [[ -f "dist" ]]; then
    rm -r dist
fi

# Create new dist
mkdir -p dist

# Build
cargo build --target wasm32-unknown-unknown  --release

# Generate bindgen outputs
wasm-bindgen target/wasm32-unknown-unknown/release/$PROJECT_NAME.wasm --out-dir dist --target web --no-typescript

# Create index from the HTML variable
echo "$HTML" > dist/index.html

# Copy assets
if [[ -f "assets" ]]; then
    cp -r assets dist/
fi
