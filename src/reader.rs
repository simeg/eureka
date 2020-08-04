use std::io;

pub trait ReadInput {
    fn read_input(&mut self) -> String;
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
    fn read_input(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::{ReadInput, Reader};

    #[test]
    fn test_reader_works() {
        let input = b"  some input  ";
        let mut reader = Reader { reader: &input[..] };

        let actual = reader.read_input();
        let expected = "some input";

        assert_eq!(actual, expected);
    }
}
