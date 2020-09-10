#!/bin/bash

# This build script is useful mostly for testing purposes, because `cargo test`
# won't work on this crate. This is because the code in this crate is intended
# to run on the wasm32-unknown-unknown target.

function log() {
    echo "[build.sh] $*";
}

PROJECT_DIR="$( cd "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"

mode=$1
target=$2

case $mode in
    "rel" | "release" | "prod" | "production")
        case $target in
            "node" | "nodejs" | "NodeJS")
                npm i                                       &&
                wasm-pack build --release --target nodejs
                ;;
            "browser" | "browsers" | "bundler")
                npm i                                       &&
                wasm-pack build --release --target bundler
                ;;
            *)
                log "Unknown argument(s): $0 $mode >$target<"
            ;;
        esac
        ;;
    "dev" | "development")
        case $target in
            "node" | "nodejs" | "NodeJS")
                npm i                                       &&
                    wasm-pack build --dev --target nodejs
                ;;
            "browser" | "browsers" | "bundler")
                npm i                                       &&
                    wasm-pack build --dev --target bundler
                ;;
            *)
                log "Unknown argument(s): $0 $mode >$target<"
                ;;
        esac
        ;;
    *)
        log "Unknown argument(s): $0 >$mode< $target"
        ;;
esac
