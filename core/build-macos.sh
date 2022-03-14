#!/bin/bash

set -eoux pipefail

XCFRAMEWORK=HimawariRenderCore.xcframework

cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

lipo -create \
  target/x86_64-apple-darwin/release/libhimawari_render.a \
  target/aarch64-apple-darwin/release/libhimawari_render.a \
  -output target/libhimawari_macos.a

lipo -info target/libhimawari_macos.a

xcodebuild -create-xcframework \
  -library ./target/libhimawari_macos.a \
  -headers ./include/ \
  -output target/HimawariRenderCore.xcframework

zip -r target/$XCFRAMEWORK.zip target/$XCFRAMEWORK

openssl dgst -sha256 target/$XCFRAMEWORK.zip