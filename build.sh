#!/bin/bash

# Regular build
cargo build --release

export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=/usr/bin/arm-linux-gnueabihf-gcc

# Pi 0/1
cargo build --release --target=arm-unknown-linux-gnueabihf

# Pi 2/3/4
cargo build --release --target=armv7-unknown-linux-gnueabihf

VERSION=$(./target/release/tcpc -V | cut -d " " -f2)
BUILD_DIR="./build/${VERSION}"

mkdir -p ${BUILD_DIR}

cp ./target/release/tcpc ${BUILD_DIR}/tcpc_${VERSION}_linux_amd64
cp ./target/arm-unknown-linux-gnueabihf/release/tcpc ${BUILD_DIR}/tcpc_${VERSION}_linux_arm
cp ./target/armv7-unknown-linux-gnueabihf/release/tcpc ${BUILD_DIR}/tcpc_${VERSION}_linux_armv7

function pack() {
    NAME=$1
    cd ${BUILD_DIR}
    cp tcpc_${VERSION}_${NAME} tcpc
    zip tcpc_${VERSION}_${NAME}.zip tcpc &>/dev/null
    rm tcpc
    cd - &>/dev/null
}

pack linux_arm
pack linux_amd64
pack linux_armv7
