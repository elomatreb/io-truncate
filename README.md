# `io-truncate`

A trait for IO objects that can be shortened (truncated).

## Example

```rust
use io_truncate::Truncate;

let mut v: &[u8] = &[0, 1, 2, 3];
v.truncate(3).unwrap();
assert_eq!(v, &[0, 1, 2]);
```

## License

Licensed under either of

 - Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 - MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

