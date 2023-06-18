# Easy Hex

An easy to use Hex string formatting wrapper.

## Features

- serde
- formatting
- flexible, everything that has TryFrom or AsRef
- transparent

# TODO

- list all features

# Examples

Serializing byte vectors as hex strings:

```rust
use easy_hex::Hex;
use serde::{Serialize, Deserialize};
# use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Example {
    // Fixed size data
    array: Hex<[u8; 16]>,
    // Dynamic sized data
    vec: Hex<Vec<u8>>,
    // Without wrapper
    #[serde(with = "easy_hex::serde")]
    boxed: Box<[u8]>
}

```

Formatting bytes as hex:

```rust
use easy_hex::{Hex, HexExt};

let data: [u8; 4] = [222, 173, 190, 239];

// Format by creating a temporary `&Hex<[u8; N]>`
let out = format!("contents of data: {}", data.as_hex());
assert_eq!(out, "contents of data: deadbeef");

// Format by wrapping the data explicitly
let hex = Hex(data);
println!("display: {hex}");
println!("debug: {hex:?}");
println!("explicit lower: {hex:x}");
println!("explicit upper: {hex:X}");
```
