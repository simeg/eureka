use std::io;

pub trait ReadInput {
    fn read_input(&mut self) -> io::Result<String>;
}

pub struct Reader<R> {
    reader: R,
}

impl<R> Reader<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R: io::BufRead> ReadInput for Reader<R> {
    fn read_input(&mut self) -> io::Result<String> {
        let mut input = String::new();
        self.reader.read_line(&mut input)?;
        Ok(input.trim().to_string())
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use crate::reader::{ReadInput, Reader};

    #[test]
    fn test_reader__read_input__success() {
        let input = b"  my input with whitespace chars  ";
        let mut reader = Reader::new(&input[..]);

        let actual = reader.read_input().unwrap();
        let expected = "my input with whitespace chars".to_string();

        assert_eq!(actual, expected);
    }
}
