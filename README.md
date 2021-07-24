# nonzero: Statically Checked Non-Zero Integers

The crate provides the `nonzero!` macro that converts an integer to `NonZero{Usize,Isize,...}` types. The conversion is done statically without extra runtime cost, and rejects zeros in compile-time.

\[ [docs.rs](https://docs.rs/nonzero) | [crates.io](https://crates.io/crates/nonzero) \]

## Example

```rust
use nonzero::nonzero as nz;
use std::num::{NonZeroI32, NonZeroUsize};

let safe_seven: NonZeroUsize = nz!(7usize);
let negative_one: NonZeroI32 = nz!(-1i32);
```

## License

MIT license. See [license file](LICENSE.txt).
