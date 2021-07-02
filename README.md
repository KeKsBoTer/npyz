# npyz

[![crates.io version](https://img.shields.io/crates/v/npyz.svg)](https://crates.io/crates/npyz) [![Documentation](https://docs.rs/npyz/badge.svg)](https://docs.rs/npyz/) [![Build Status](https://github.com/ExpHP/npyz/actions/workflows/ci.yml/badge.svg)](https://github.com/ExpHP/npyz/actions)

Numpy format (`*.npy`) serialization and deserialization.

[**NPY**](https://docs.scipy.org/doc/numpy-dev/neps/npy-format.html) is a simple binary data format.
It stores the type, shape and endianness information in a header,
which is followed by a flat binary data field. This crate offers a simple, mostly type-safe way to
read and write `*.npy` files. Files are handled using iterators, so they don't need to fit in memory.

`npyz` is a fork and successor of the seemingly-dead [`npy`](https://github.com/potocpav/npy-rs).

## Usage

```toml
npyz = "0.5"
```

You also may be interested in enabling some features:

```toml
npyz = {version = "0.5", features = ["derive", "complex"]}
```

Data can now be read from a `*.npy` file:

```rust
use npyz::NpyReader;

fn main() -> std::io::Result<()> {
    let bytes = std::fs::read("examples/plain.npy")?;

    // Note: In addition to byte slices, this accepts any io::Read
    let data: NpyReader<f64, _> = NpyReader::new(&bytes[..])?;
    for number in data {
        let number = number?;
        eprintln!("{}", number);
    }
    Ok(())
}
```

For further examples and information on:
* Reading `npy` files,
* Writing `npy` files,
* Working with structured arrays,
* Interop with the `ndarray` crate,

please see the [documentation on the root module](https://docs.rs/npyz).

## Relation to similar crates

The name `npyz` is actually an abbreviation.  Here is the full name of the crate:

> `npy` plus npz support, and a lot of other features that are frankly a lot more important than npz—not to mention the fact that npz support isn't even actually included in the first release—but I had to call it something, okay

To clarify, `npyz` is a fork of Pavel Potoček's [`npy` crate](https://github.com/potocpav/npy-rs).  The original `npy` supported structured arrays with derives, but had many, many limitations:

* 1D arrays only.
* Little endian only.
* No `Complex`, no bytestrings.
* Reading API based on `&[u8]` instead of `Read`, with the expectation that a user can use a memmap for files too large to fit in memory.
* No `io::Write`, only one writing API that writes directly to the filesystem.

Originally, ~~`nippy`~~ `npyz` was a place for me to protype new features with reckless abandon before finally making a PR to `npy`, but even my first few foundational PRs have yet to be merged upstream.  I believe Pavel has a good head on their shoulders and a great attention to detail, and I appreciated their initial response on my PRs, but nearly two years have passed since the last time I have heard from them. Therefore, I've decided to go forward and publish the fork.

## License

`npyz` is Copyright 2021 Michael Lamparski, and provided under the terms of the MIT License.

`npyz` is based off of `npy`.  `npy` is Copyright 2018 Pavel Potoček, which was provided under the terms of the MIT License.
