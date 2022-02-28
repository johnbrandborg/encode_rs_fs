# encoding_rs_fs
Read and Write files with encoding_rs.

### Documentation

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
encoding_rs_fs = "0.1"
```

and this to your crate root:

```rust
extern crate encoding_rs_io;
```

For a list of encodings refer to the documentions at https://docs.rs/encoding_rs

### Example

Use the functions to read and write entire files using a encoding.

```rust
extern crate encoding_rs_fs;

use::encoding_rs_fs::{read, write};

fn main() {
    let test_file = "example.txt";
    let source = "ÁáAaBbCc";
    let codec = "latin1";

    write(test_file, source, codec).unwrap();
    let result = read(test_file, codec).unwrap();
    println!("Results {:?}", result);
}
```
