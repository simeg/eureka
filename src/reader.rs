use std::io;

pub struct Reader<W> {
    pub reader: W,
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

#[cfg(test)]
mod tests {
    use super::*;

    struct MockReader<R> {
        pub reader: R,
    }

    impl<R: io::BufRead> Read for MockReader<R> {
        fn read(&mut self) -> String {
            unimplemented!()
        }
    }

    #[test]
    fn tests_work() {
        let stdio = io::stdin();
        let input = stdio.lock();
        let mut _r = MockReader { reader: input };

        assert_eq!(true, true);
    }
}
