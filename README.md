Experiments over dynamic plugins in Rust

Usage:

```sh
% cargo build
% export LD_LIBRARY_PATH="$HOME/.rustup/toolchains/$(rustup toolchain list| grep default | cut -d' ' -f1)/lib"
% ./target/debug/application target/debug/libplugin.so
```
