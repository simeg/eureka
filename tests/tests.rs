#[cfg(test)]
mod tests {
    use eureka::file_handler::{ConfigManagement, FileManagement};
    use eureka::printer::{Print, PrintColor, PrintOptions};
    use eureka::reader::ReadInput;
    use eureka::types::ConfigFile;
    use eureka::{Eureka, EurekaOptions};

    use atomic_counter::{AtomicCounter, RelaxedCounter};
    use std::io;
    use std::io::ErrorKind;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::Arc;

    struct DefaultMockPrinter;

    impl Print for DefaultMockPrinter {
        fn print(&mut self, _value: &str) {
            unimplemented!()
        }

        fn println(&mut self, _value: &str) {
            unimplemented!()
        }
    }

    impl PrintColor for DefaultMockPrinter {
        fn fts_banner(&mut self) {
            unimplemented!()
        }

        fn input_header(&mut self, _value: &str) {
            unimplemented!()
        }

        fn println_styled(&mut self, _value: &str, _opts: PrintOptions) {
            unimplemented!()
        }
    }

    struct DefaultMockReader;

    impl ReadInput for DefaultMockReader {
        fn read_input(&mut self) -> String {
            unimplemented!()
        }
    }

    #[test]
    fn test_clear_repo() {
        struct MockFileHandler;

        impl ConfigManagement for MockFileHandler {
            fn config_dir_create(&self) -> io::Result<String> {
                unimplemented!()
            }

            fn config_dir_exists(&self) -> bool {
                unimplemented!()
            }

            fn config_read(&self, file: ConfigFile) -> io::Result<String> {
                assert_eq!(file, ConfigFile::Repo);
                Ok("some-path".to_string())
            }

            fn config_write(&self, _file: ConfigFile, _value: String) -> io::Result<()> {
                unimplemented!()
            }
        }

        impl FileManagement for MockFileHandler {
            fn file_rm(&self, file: ConfigFile) -> io::Result<()> {
                assert_eq!(file, ConfigFile::Repo);
                Ok(())
            }
        }

        let mut eureka = Eureka::new(
            MockFileHandler {},
            DefaultMockPrinter {},
            DefaultMockReader {},
        );
        let opts = EurekaOptions {
            clear_repo: true,
            clear_branch: false,
            view: false,
        };

        let actual = eureka.run(opts);

        assert!(actual.is_ok());
    }

    #[test]
    fn test_clear_branch() {
        struct MockFileHandler;

        impl ConfigManagement for MockFileHandler {
            fn config_dir_create(&self) -> io::Result<String> {
                unimplemented!()
            }

            fn config_dir_exists(&self) -> bool {
                unimplemented!()
            }

            fn config_read(&self, file: ConfigFile) -> io::Result<String> {
                assert_eq!(file, ConfigFile::Branch);
                Ok("some-path".to_string())
            }

            fn config_write(&self, _file: ConfigFile, _value: String) -> io::Result<()> {
                unimplemented!()
            }
        }

        impl FileManagement for MockFileHandler {
            fn file_rm(&self, file: ConfigFile) -> io::Result<()> {
                assert_eq!(file, ConfigFile::Branch);
                Ok(())
            }
        }

        let mut eureka = Eureka::new(
            MockFileHandler {},
            DefaultMockPrinter {},
            DefaultMockReader {},
        );
        let opts = EurekaOptions {
            clear_repo: false,
            clear_branch: true,
            view: false,
        };

        let actual = eureka.run(opts);

        assert!(actual.is_ok());
    }

    #[test]
    fn test_clear_repo_and_branch() {
        struct MockFileHandler;
        static READ_COUNTER: AtomicUsize = AtomicUsize::new(0);
        static RM_COUNTER: AtomicUsize = AtomicUsize::new(0);

        impl ConfigManagement for MockFileHandler {
            fn config_dir_create(&self) -> io::Result<String> {
                unimplemented!()
            }

            fn config_dir_exists(&self) -> bool {
                unimplemented!()
            }

            fn config_read(&self, file: ConfigFile) -> io::Result<String> {
                let counter = READ_COUNTER.fetch_add(1, Ordering::SeqCst);
                if counter == 0 {
                    assert_eq!(file, ConfigFile::Repo);
                } else if counter == 1 {
                    assert_eq!(file, ConfigFile::Branch);
                }

                Ok("some-path".to_string())
            }

            fn config_write(&self, _file: ConfigFile, _value: String) -> io::Result<()> {
                unimplemented!()
            }
        }

        impl FileManagement for MockFileHandler {
            fn file_rm(&self, file: ConfigFile) -> io::Result<()> {
                let counter = RM_COUNTER.fetch_add(1, Ordering::SeqCst);
                if counter == 0 {
                    assert_eq!(file, ConfigFile::Repo);
                } else if counter == 1 {
                    assert_eq!(file, ConfigFile::Branch);
                }

                Ok(())
            }
        }

        let mut eureka = Eureka::new(
            MockFileHandler {},
            DefaultMockPrinter {},
            DefaultMockReader {},
        );
        let opts = EurekaOptions {
            clear_repo: true,
            clear_branch: true,
            view: false,
        };

        let actual = eureka.run(opts);

        assert!(actual.is_ok());
    }

    #[test]
    #[ignore]
    fn test_view_ideas() {
        // TODO: Need to figure out how to mock open_pager()
        struct MockFileHandler;

        impl ConfigManagement for MockFileHandler {
            fn config_dir_create(&self) -> io::Result<String> {
                unimplemented!()
            }

            fn config_dir_exists(&self) -> bool {
                unimplemented!()
            }

            fn config_read(&self, file: ConfigFile) -> io::Result<String> {
                assert_eq!(file, ConfigFile::Repo);
                Ok("some-path".to_string())
            }

            fn config_write(&self, _file: ConfigFile, _value: String) -> io::Result<()> {
                unimplemented!()
            }
        }

        impl FileManagement for MockFileHandler {
            fn file_rm(&self, _file: ConfigFile) -> io::Result<()> {
                // let counter = RM_COUNTER.fetch_add(1, Ordering::SeqCst);
                // if counter == 0 {
                //     assert_eq!(file, ConfigFile::Repo);
                // } else if counter == 1 {
                //     assert_eq!(file, ConfigFile::Branch);
                // }

                Ok(())
            }
        }

        let mut eureka = Eureka::new(
            MockFileHandler {},
            DefaultMockPrinter {},
            DefaultMockReader {},
        );
        let opts = EurekaOptions {
            clear_repo: true,
            clear_branch: true,
            view: false,
        };

        let actual = eureka.run(opts);

        assert!(actual.is_ok());
    }

    #[test]
    fn test_config_dir_is_missing() {
        struct MockFileHandler;

        impl ConfigManagement for MockFileHandler {
            fn config_dir_create(&self) -> io::Result<String> {
                Ok("arbitrary-string".to_string())
            }

            fn config_dir_exists(&self) -> bool {
                false
            }

            fn config_read(&self, _file: ConfigFile) -> io::Result<String> {
                let counter = self.config_read_counter.fetch_add(1, Ordering::SeqCst);
                if counter == 0 {
                    Err(io::Error::new(ErrorKind::Other, "arbitrary-error"))
                } else {
                    Ok("arbitrary-string".to_string())
                }
            }

            fn config_write(&self, _file: ConfigFile, _value: String) -> io::Result<()> {
                unimplemented!()
            }
        }

        impl FileManagement for MockFileHandler {
            fn file_rm(&self, _file: ConfigFile) -> io::Result<()> {
                unimplemented!()
            }
        }

        struct MockPrinter;

        impl Print for MockPrinter {
            fn print(&mut self, value: &str) {
                assert_eq!(value, "First time setup complete. Happy ideation!");
            }

            fn println(&mut self, _value: &str) {
                unimplemented!()
            }
        }

        impl PrintColor for MockPrinter {
            fn fts_banner(&mut self) {
                self.fts_banner_called.store(true, Ordering::SeqCst);
            }

            fn input_header(&mut self, _value: &str) {
                unimplemented!()
            }

            fn println_styled(&mut self, _value: &str, _opts: PrintOptions) {
                unimplemented!()
            }
        }

        let fh = MockFileHandler {};

        let printer = MockPrinter {};

        let mut eureka = Eureka::new(fh, printer, DefaultMockReader {});
        let opts = EurekaOptions {
            clear_repo: false,
            clear_branch: false,
            view: false,
        };

        let actual = eureka.run(opts);
        assert!(actual.is_ok());

        // let actual_create_dir_counter = fh.config_dir_create_counter.into_inner();
        // let expected_create_dir_counter = 1 as usize;
        //
        // assert_eq!(actual_create_dir_counter, expected_create_dir_counter);
        //
        // let actual_config_read_counter = CONFIG_READ_COUNTER.load(Ordering::SeqCst);
        // let expected_config_read_counter = 3;
        //
        // assert_eq!(actual_config_read_counter, expected_config_read_counter);
        //
        // let actual_fts_banner_called = FTS_BANNER_CALLED.load(Ordering::SeqCst);
        // assert!(actual_fts_banner_called);
    }
}
