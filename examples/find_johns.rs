use large_json_array::JsonStream;
use serde_json::Value;
use std::io::BufReader;
use std::time::Instant;
mod common;
use common::helpers::*;

fn main() {
    let mut john_count = 0;
    let mut user_count = 0;
    let start_time = Instant::now();

    let file: std::fs::File = load_from_env_or_ask();
    let reader = BufReader::new(file);
    let stream = JsonStream::new(reader); // the streamer in used here

    // iteration over the stream
    for item in stream {
        let val: Value = item.unwrap(); // unwrap or propagate any JSON‐parsing errors
        let name = val["name"].as_str().unwrap();
        if name.starts_with("John") {
            john_count += 1;
        }
        user_count += 1;
    }
    println!("\ntotal users: {}", user_count);
    println!("time elapsed: {:?}", start_time.elapsed());
    println!("found {} Johns", john_count);
}
