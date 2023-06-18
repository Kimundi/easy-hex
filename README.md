# Easy Hex

An easy to use Hex-String formatting wrapper.

The goal of this library is to make it very easy to format or serialize any
container of bytes as a hexadecimal string. This includes vectors, arrays,
slices, etc. Example:

```rust
# use easy_hex::Hex;
let hex = Hex([1, 16, 255]);
let json = serde_json::to_string(&hex).unwrap();
assert_eq!(json, r#""0110ff""#);
```

## Features

- Compatible with every type that implements
  `TryFrom<&[u8]>` and `AsRef<[u8]>`.
- Flexible API, can be used with both owned and borrowed byte containers.
- Transparent type representation, allows freely casting between wrapper and
  wrapped type.
- Can wrap dynamically sized types.
- Supports lowercase and uppercase hex.
- Supports `serde`: Any byte container can be easily serialized as
  a hex string.
- Supports `std` formatting: Any byte container can be easily formatted as
  a hex string.
- Supports `bytemuck`: Allows safely casting between wrapper and wrapped type,
  and allows casting of references to the wrapper.

## Supported Types

The API of this crate aims to support as many types as possible:

- Serialization is supported for all `T: AsRef<[u8]>`.
- Deserialization is supported for all `T: TryFrom<&[u8]>`.

This covers, among other things, these types:

- `Vec<u8>`
- `[u8]`
- `[u8; N]`
- `Box<[u8]>`
- `&[u8]`
- Other referenced types like `&Vec<u8>`
- Mutable referenced types like `&mut [u8; N]`
- Third Party types like `GenericArray<u8, N>` (Although this type currently only supports serialization, due to a missing `TryFrom` impl)

Note the explicit support of dynamically sized types like `[u8]`.
They are possible because of the `transparent` representation:

```rust
# use easy_hex::Hex;
let data: &[u8] = &[1, 2, 3];
let hex: &Hex<[u8]> = data.into();
```

## Relevancy

There are many hex string formatting crates already, and this one does
not aim to be better than any of them.

The main reason this exists is that the author wanted a reusable crate
for hex string serialization with minimal boilerplate.

## Performance

No particular performance benchmarks or optimizations have been applied to
this crate so far. The properties of the implementation are:

- It reuses the encoding and decoding implementation of the `hex` crate.
- It uses a stack buffer for small hexstrings or byte sequences, but otherwise
  needs to allocate a temporary vector during transcoding.

## More Examples

Serializing byte vectors as hex strings:

```rust
use easy_hex::Hex;
use serde::{Serialize, Deserialize};
# use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Example {
    // With wrapper
    array: Hex<[u8; 16]>,
    // Without wrapper
    #[serde(with = "easy_hex::serde")]
    vec: Vec<u8>
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
