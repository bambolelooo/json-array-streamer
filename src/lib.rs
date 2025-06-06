pub mod error;
pub mod json_stream;

pub use error::JsonError;
pub use json_stream::JsonStream;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_array() {
        let data = r#"[{"foo":1}, {"bar":2}]"#;
        let reader = std::io::Cursor::new(data);
        let mut it = JsonStream::new(reader);
        assert_eq!(it.next().unwrap().unwrap()["foo"], 1);
        assert_eq!(it.next().unwrap().unwrap()["bar"], 2);
        assert!(it.next().is_none());
    }
}
