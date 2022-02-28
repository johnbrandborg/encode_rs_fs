# encode_rs_fs
Read and Write files using encodings.

### Documentation

Offical documentation can be found at https://docs.rs/encode_rs_fs

I think I would have preferred the library to be called `encoding_rs_fs`, but
once I publishing to crate to [crates.io](https://crates.io/crates/encode_rs_fs)
it was to late, and, it can't be changed.

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
encode_rs_fs = "0.1"
```

and this to your crate root:

```rust
extern crate encode_rs_fs;
```

For a full list of encodings that can be used refer to the documentions at
[Docs.rs](https://docs.rs/encoding_rs).

### Example

Use the functions to read and write entire files using a encoding.

```rust
extern crate encodingfs;

use::encode_rs_fs::{read, write};

fn main() {
    let test_file = "example.txt";
    let source = "ÁáAaBbCc";
    let codec = "latin1";

    write(test_file, source, codec).unwrap();
    let result = read(test_file, codec).unwrap();
    println!("Results {:?}", result);
}
```
