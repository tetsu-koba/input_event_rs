#!/bin/sh
TARGET=input_event
STRIP=aarch64-buildroot-linux-gnu-strip
export SYSROOT=/home/koba/arm64/buildroot-2018.05/output/host/usr/aarch64-buildroot-linux-gnu/sysroot/
export PKG_CONFIG_ALLOW_CROSS=1
cargo build --target aarch64-unknown-linux-gnu --release
cd target/aarch64-unknown-linux-gnu/release/
cp $TARGET ${TARGET}_g
$STRIP $TARGET
