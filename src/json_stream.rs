use crate::error::JsonError;
use serde_json::Value;
use std::collections::VecDeque;
use std::io::Read;

pub struct JsonStream<R: Read> {
    reader: R,
    buffer: Vec<u8>,
    temp: VecDeque<u8>, // leftover chars not yet processed
    in_string: bool,
    brace_count: u16,
    inside_array: bool,
    object_buffer: String, // characters of the current object
}

impl<R: Read> JsonStream<R> {
    pub fn new(reader: R) -> Self {
        JsonStream {
            reader,
            buffer: vec![0; 1024],
            temp: VecDeque::new(),
            in_string: false,
            brace_count: 0,
            inside_array: false,
            object_buffer: String::new(),
        }
    }
    pub fn find_object_in_buffer(&mut self) -> Option<Result<Value, JsonError>> {
        while let Some(c) = self.temp.pop_front() {
            if self.brace_count > 0 || (c == b'{' && !self.in_string) {
                self.object_buffer.push(char::from(c));
            }

            match c as u8 {
                b'"' => {
                    let mut backslashes = 0;
                    while self.object_buffer.chars().rev().nth(backslashes) == Some('\\') {
                        backslashes += 1;
                    }
                    if backslashes % 2 == 0 {
                        self.in_string = !self.in_string;
                    }
                }
                b'[' if !self.in_string && !self.inside_array => {
                    self.inside_array = true;
                    self.object_buffer.clear(); // drop the '['
                    continue;
                }
                b'{' if !self.in_string => {
                    self.brace_count += 1;
                }
                b'}' if !self.in_string => {
                    self.brace_count -= 1;
                    if self.brace_count == 0 {
                        if self.object_buffer.trim().is_empty() {
                            self.object_buffer.clear();
                            continue;
                        }

                        let obj_str = self.object_buffer.clone();

                        self.object_buffer.clear();

                        while let Some(next_ch) = self.temp.front() {
                            if next_ch.is_ascii_whitespace() || *next_ch == b',' {
                                self.temp.pop_front();
                            } else {
                                break;
                            }
                        }
                        return Some(serde_json::from_str(&obj_str).map_err(JsonError::from));
                    }
                }
                _ => {}
            }
        }

        None
    }
}

impl<R: Read> Iterator for JsonStream<R> {
    type Item = Result<Value, JsonError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(obj) = self.find_object_in_buffer() {
            return Some(obj);
        }
        loop {
            match self.reader.read(&mut self.buffer) {
                Ok(0) => {
                    return None;
                }
                Ok(n) => {
                    self.temp.extend(&self.buffer[..n]);
                    if let Some(obj) = self.find_object_in_buffer() {
                        return Some(obj);
                    }
                }
                Err(e) => {
                    return Some(Err(JsonError::Io(e)));
                }
            }
        }
    }
}
