use crate::error::JsonError;
use serde_json::Value;
use std::io::Read;
use std::str;

pub struct JsonStream<R: Read> {
    reader: R,
    buffer: Vec<u8>,
    temp: String, // leftover chars not yet processed
    in_string: bool,
    brace_count: u16,
    inside_array: bool,
    object_buffer: String, // accumulates characters of the current object
}

impl<R: Read> JsonStream<R> {
    pub fn new(reader: R) -> Self {
        JsonStream {
            reader,
            buffer: vec![0; 1024],
            temp: String::new(),
            in_string: false,
            brace_count: 0,
            inside_array: false,
            object_buffer: String::new(),
        }
    }
    pub fn find_object_in_buffer(&mut self) -> Option<Result<Value, JsonError>> {
        let mut chars = self.temp.chars().peekable();
        while let Some(c) = chars.next() {
            if self.brace_count > 0 || (c == '{' && !self.in_string) {
                self.object_buffer.push(c);
            }

            match c {
                '"' => {
                    // toggle in_string unless escaped
                    let mut backslashes = 0;
                    while self.object_buffer.chars().rev().nth(backslashes) == Some('\\') {
                        backslashes += 1;
                    }
                    if backslashes % 2 == 0 {
                        self.in_string = !self.in_string;
                    }
                }
                '[' if !self.in_string && !self.inside_array => {
                    self.inside_array = true;
                    self.object_buffer.clear(); // drop the '['
                    continue;
                }
                '{' if !self.in_string => {
                    self.brace_count += 1;
                }
                '}' if !self.in_string => {
                    self.brace_count -= 1;
                    if self.brace_count == 0 {
                        // We have one complete object in `object_buffer`.
                        if self.object_buffer.trim().is_empty() {
                            // e.g. empty braces or spurious; just clear and continue
                            self.object_buffer.clear();
                            continue;
                        }

                        // 1) take out the exact JSON‐text for this object
                        let obj_str = self.object_buffer.clone();

                        // 2) clear the buffer for the next object
                        self.object_buffer.clear();

                        // 3) skip any commas or whitespace after this object
                        while let Some(next_ch) = chars.peek() {
                            if next_ch.is_whitespace() || *next_ch == ',' {
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        // add remaining chars to buffer
                        self.temp = chars.collect();

                        return Some(serde_json::from_str(&obj_str).map_err(JsonError::from));
                    }
                }
                _ => {}
            }
        }

        // temp gets only unprocessed remainder
        self.temp = chars.collect();
        None
    }
}

impl<R: Read> Iterator for JsonStream<R> {
    type Item = Result<Value, JsonError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(obj) = self.find_object_in_buffer() {
            return Some(obj);
        }
        while let Ok(n) = self.reader.read(&mut self.buffer) {
            if n == 0 {
                return None;
            }
            let chunk = str::from_utf8(&self.buffer[..n]).unwrap(); // assumes UTF-8 JSON
            self.temp.push_str(chunk);
            return self.find_object_in_buffer();
        }
        Some(Err(JsonError::Parser))
    }
}
