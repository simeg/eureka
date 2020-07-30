use std::io;

pub trait Read {
    fn read(&mut self) -> String;
}

pub struct Reader<R> {
    pub reader: R,
}

impl<R: io::BufRead> Read for Reader<R> {
    fn read(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::{Read, Reader};

    #[test]
    fn test_reader_works() {
        let input = b"  some input  ";
        let mut reader = Reader { reader: &input[..] };

        let actual = reader.read();
        let expected = "some input";

        assert_eq!(actual, expected);
    }
}
