1. build release 构建发行版本
cargo build --release

2. build uf2
link https://github.com/JoNil/elf2uf2-rs

cd .\target\thumbv6m-none-eabi\release\

elf2uf2-rs rust-pico rust-fly.uf2