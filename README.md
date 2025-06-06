# large-json-array

**Stream large JSON arrays in Rust using an efficient iterator-based approach.**

## 🚀 Overview

Serde's default behavior loads entire JSON documents into memory. When dealing with *very large JSON arrays*, this
becomes inefficient or outright impossible. This crate provides a custom `JsonStream<R: Read>` iterator that streams and
parses objects from a JSON array one-by-one without allocating the entire array in memory.

## 🔧 Features

- Memory-efficient streaming of large JSON arrays
- Works with any `Read` source (e.g., files, network streams)
- Integrates with `serde_json::Value`
- Robust bracket tracking and string-state handling

## 🛠 Example

```rust
use std::fs::File;
use std::io::BufReader;
use large_json_array::json_stream::JsonStream;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("huge.json")?;
    let reader = BufReader::new(file);
    let stream = JsonStream::new(reader);

    for value in stream {
        match value {
            Ok(json_value) => println!("{:?}", json_value),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}
````

## 📦 Crate Structure

```
src/
├── bin/               # Optional binaries
├── error.rs           # Custom error type `JsonError`
├── json_stream.rs     # Streaming iterator implementation
├── lib.rs             # Library entry point
```

## ⚙️ How It Works

The core struct is:

```rust
pub struct JsonStream<R: Read> {
    ...
}
```

It implements `Iterator<Item = Result<Value, JsonError>>` and maintains:

* A buffer for reading chunks
* A character accumulator for partial objects
* Bracket and string context state

Parsing logic handles array delimiters, quoted strings, escape sequences, and ensures objects are emitted only when
fully formed.

## 📥 Installation

In your `Cargo.toml`:

```toml
[dependencies]
large-json-array = { git = "https://github.com/bambolelooo/large-json-array" }
```

## ❗ Limitations

* Assumes well-formed JSON arrays of objects (e.g., `[ {...}, {...}, ... ]`)
* Strings must be UTF-8 encoded
* Not suitable for deeply malformed JSON (for now)

## 📄 License

This project is licensed under the terms of the MIT license.

---

For more advanced use cases or improvements (e.g., error recovery, typed deserialization, or multi-threaded parsing),
contributions and ideas are welcome!

