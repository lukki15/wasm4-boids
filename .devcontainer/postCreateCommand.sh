#!/bin/sh

# get wasm4
wget https://github.com/aduros/wasm4/releases/latest/download/w4-linux.zip
unzip w4-linux.zip
rm w4-linux.zip
mv w4 /usr/bin

# get wasm-opt
apt update
apt install binaryen
