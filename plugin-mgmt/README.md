# plugin-mgmt-0.1.0

Dynamic plugin management helper. Experiments over dynamic plugins in Rust.

To see the example in action (on linux):

```sh
% cd $(git rev-parse --show-toplevel)
% cargo build
% ./target/debug/example-application ./target/debug/libplugin_example.so
```

## Allocations

By default the library enforces the
[`System`](https://doc.rust-lang.org/std/alloc/struct.System.html) allocator to be used by
setting it explicitly via
[`#[global_allocator]`](https://doc.rust-lang.org/std/alloc/index.html#the-global_allocator-attribute).

If this behaviour is undesired and you know what you're doing, use this library with a feature
`no_enforce_system_allocator` activated.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
