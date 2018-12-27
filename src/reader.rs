use std::io;

pub struct Reader<R> {
    pub reader: R,
}

pub trait Read {
    fn read(&mut self) -> String;
}

impl<R: io::BufRead> Read for Reader<R> {
    fn read(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}
