#[cfg(test)]
mod tests {
    use eureka::file_handler::{ConfigManagement, FileManagement};
    use eureka::printer::{Print, PrintColor, PrintOptions};
    use eureka::reader::ReadInput;
    use eureka::types::ConfigFile;
    use eureka::Eureka;

    use std::io;

    struct MockFileHandler;

    impl ConfigManagement for MockFileHandler {
        fn config_dir_create(&self) -> io::Result<String> {
            unimplemented!()
        }

        fn config_dir_exists(&self) -> bool {
            unimplemented!()
        }

        fn config_read(&self, file: ConfigFile) -> io::Result<String> {
            unimplemented!()
        }

        fn config_write(&self, file: ConfigFile, value: String) -> io::Result<()> {
            unimplemented!()
        }
    }

    impl FileManagement for MockFileHandler {
        fn file_rm(&self, file: ConfigFile) -> io::Result<()> {
            unimplemented!()
        }
    }

    struct MockPrinter;

    impl Print for MockPrinter {
        fn print(&mut self, value: &str) {
            unimplemented!()
        }

        fn println(&mut self, value: &str) {
            unimplemented!()
        }
    }

    impl PrintColor for MockPrinter {
        fn fts_banner(&mut self) {
            unimplemented!()
        }

        fn input_header(&mut self, value: &str) {
            unimplemented!()
        }

        fn println_styled(&mut self, value: &str, opts: PrintOptions) {
            unimplemented!()
        }
    }

    struct MockReader;

    impl ReadInput for MockReader {
        fn read_input(&mut self) -> String {
            unimplemented!()
        }
    }

    #[test]
    fn test_todo() {
        let fh = MockFileHandler {};
        let printer = MockPrinter {};
        let reader = MockReader {};
        let eureka = Eureka::new(fh, printer, reader);
    }
}
