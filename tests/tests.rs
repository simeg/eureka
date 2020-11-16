#[cfg(test)]
mod tests {
    use eureka::file_handler::{ConfigManagement, FileManagement};
    use eureka::printer::{Print, PrintColor, PrintOptions};
    use eureka::reader::ReadInput;
    use eureka::types::ConfigFile;
    use eureka::{Eureka, EurekaOptions};

    use eureka::git::GitManagement;
    use eureka::program_access::ProgramOpener;
    use git2::Oid;
    use std::io;
    use std::io::{Error, ErrorKind};
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_clear_repo() {
        struct MockFileHandler;
        static READ_COUNTER: AtomicUsize = AtomicUsize::new(0);

        impl ConfigManagement for MockFileHandler {
            fn config_dir_create(&self) -> io::Result<String> {
                unimplemented!()
            }

            fn config_dir_exists(&self) -> bool {
                unimplemented!()
            }

            fn config_read(&self, file: ConfigFile) -> io::Result<String> {
                let counter = READ_COUNTER.fetch_add(1, Ordering::SeqCst);
                if counter > 0 {
                    panic!("Should only be read once");
                }

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
            DefaultGit {},
            DefaultMockProgramOpener {},
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
            DefaultGit {},
            DefaultMockProgramOpener {},
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
                } else {
                    panic!("Should not be read this many times")
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
            DefaultGit {},
            DefaultMockProgramOpener {},
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
    fn test_view_ideas() {
        struct MockFileHandler;
        static READ_COUNTER: AtomicUsize = AtomicUsize::new(0);

        impl ConfigManagement for MockFileHandler {
            fn config_dir_create(&self) -> io::Result<String> {
                unimplemented!()
            }

            fn config_dir_exists(&self) -> bool {
                unimplemented!()
            }

            fn config_read(&self, file: ConfigFile) -> io::Result<String> {
                let counter = READ_COUNTER.fetch_add(1, Ordering::SeqCst);
                if counter > 0 {
                    panic!("Should only be read once");
                }

                assert_eq!(file, ConfigFile::Repo);
                Ok("specific-repo-path".to_string())
            }

            fn config_write(&self, _file: ConfigFile, _value: String) -> io::Result<()> {
                unimplemented!()
            }
        }

        impl FileManagement for MockFileHandler {
            fn file_rm(&self, _file: ConfigFile) -> io::Result<()> {
                Ok(())
            }
        }

        struct MockProgramAccess;

        impl ProgramOpener for MockProgramAccess {
            fn open_editor(&self, _file_path: &str) -> io::Result<()> {
                unimplemented!()
            }

            fn open_pager(&self, file_path: &str) -> io::Result<()> {
                assert_eq!(file_path, "specific-repo-path/README.md");
                Ok(())
            }
        }

        let mut eureka = Eureka::new(
            MockFileHandler,
            DefaultMockPrinter {},
            DefaultMockReader {},
            DefaultGit {},
            MockProgramAccess,
        );
        let opts = EurekaOptions {
            clear_repo: false,
            clear_branch: false,
            view: true,
        };

        let actual = eureka.run(opts);

        assert!(actual.is_ok());
    }

    #[test]
    fn test_config_dir_is_missing() {
        struct MockFileHandler;
        static READ_COUNTER: AtomicUsize = AtomicUsize::new(0);

        impl ConfigManagement for MockFileHandler {
            fn config_dir_create(&self) -> io::Result<String> {
                Ok("some-string".to_string())
            }

            fn config_dir_exists(&self) -> bool {
                // Config dir is missing
                false
            }

            fn config_read(&self, _file: ConfigFile) -> io::Result<String> {
                let counter = READ_COUNTER.fetch_add(1, Ordering::SeqCst);
                if counter == 0 {
                    // First it checks if any config can be found and
                    // based on that it decides to create the config dir
                    Err(Error::new(ErrorKind::Other, "some-error"))
                } else {
                    Ok(String::from("some-ok"))
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
                // Do nothing
            }

            fn input_header(&mut self, _value: &str) {
                unimplemented!()
            }

            fn println_styled(&mut self, _value: &str, _opts: PrintOptions) {
                unimplemented!()
            }
        }

        let mut eureka = Eureka::new(
            MockFileHandler {},
            MockPrinter {},
            DefaultMockReader {},
            DefaultGit {},
            DefaultMockProgramOpener {},
        );
        let opts = EurekaOptions {
            clear_repo: false,
            clear_branch: false,
            view: false,
        };

        let actual = eureka.run(opts);

        assert!(actual.is_ok());
        assert!(counter_equals(3, &READ_COUNTER));
    }

    #[test]
    fn test_e2e_happy_path() {
        static PRINT_COUNTER: AtomicUsize = AtomicUsize::new(0);

        struct MockFileHandler;

        impl ConfigManagement for MockFileHandler {
            fn config_dir_create(&self) -> io::Result<String> {
                unimplemented!()
            }

            fn config_dir_exists(&self) -> bool {
                true
            }

            fn config_read(&self, file: ConfigFile) -> io::Result<String> {
                match file {
                    ConfigFile::Branch => Ok("specific-branch".to_string()),
                    ConfigFile::Repo => Ok("specific-repo".to_string()),
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
            fn print(&mut self, _value: &str) {
                unimplemented!()
            }

            fn println(&mut self, value: &str) {
                let counter = PRINT_COUNTER.fetch_add(1, Ordering::SeqCst);
                match counter {
                    0 => assert_eq!(value, "Adding and committing your new idea.."),
                    1 => assert_eq!(value, "Added and committed!"),
                    2 => assert_eq!(value, "Pushing your new idea.."),
                    3 => assert_eq!(value, "Pushed!"),
                    _ => panic!("Unknown state"),
                }
            }
        }

        impl PrintColor for MockPrinter {
            fn fts_banner(&mut self) {
                unimplemented!()
            }

            fn input_header(&mut self, value: &str) {
                assert_eq!(value, ">> Idea summary")
            }

            fn println_styled(&mut self, _value: &str, _opts: PrintOptions) {
                unimplemented!()
            }
        }

        struct MockGit;

        impl GitManagement for MockGit {
            fn init(&mut self, repo_path: &str) -> Result<(), git2::Error> {
                assert_eq!(repo_path, "specific-repo");
                Ok(())
            }

            fn checkout_branch(&self, branch_name: &str) -> Result<(), git2::Error> {
                assert_eq!(branch_name, "specific-branch");
                Ok(())
            }

            fn add(&self) -> Result<(), git2::Error> {
                Ok(())
            }

            fn commit(&self, subject: String) -> Result<Oid, git2::Error> {
                assert_eq!(subject, "read-input-string");
                Ok(Oid::zero())
            }

            fn push(&self, branch_name: &str) -> Result<(), git2::Error> {
                assert_eq!(branch_name, "specific-branch");
                Ok(())
            }
        }

        struct MockProgramOpener;

        impl ProgramOpener for MockProgramOpener {
            fn open_editor(&self, file_path: &str) -> io::Result<()> {
                assert_eq!(file_path, "specific-repo/README.md");
                Ok(())
            }

            fn open_pager(&self, _file_path: &str) -> io::Result<()> {
                unimplemented!()
            }
        }

        let mut eureka = Eureka::new(
            MockFileHandler {},
            MockPrinter {},
            DefaultMockReader {},
            MockGit {},
            MockProgramOpener {},
        );
        let opts = EurekaOptions {
            clear_repo: false,
            clear_branch: false,
            view: false,
        };

        let actual = eureka.run(opts);

        assert!(actual.is_ok());
    }

    fn counter_equals(num: u8, counter: &AtomicUsize) -> bool {
        let counter = counter.fetch_add(0, Ordering::SeqCst);
        counter == num as usize
    }

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
            String::from("read-input-string")
        }
    }

    struct DefaultMockFileHandler;

    impl ConfigManagement for DefaultMockFileHandler {
        fn config_dir_create(&self) -> io::Result<String> {
            unimplemented!()
        }

        fn config_dir_exists(&self) -> bool {
            unimplemented!()
        }

        fn config_read(&self, _file: ConfigFile) -> io::Result<String> {
            unimplemented!()
        }

        fn config_write(&self, _file: ConfigFile, _value: String) -> io::Result<()> {
            unimplemented!()
        }
    }

    impl FileManagement for DefaultMockFileHandler {
        fn file_rm(&self, _file: ConfigFile) -> io::Result<()> {
            unimplemented!()
        }
    }

    struct DefaultGit;

    impl GitManagement for DefaultGit {
        fn init(&mut self, _repo_path: &str) -> Result<(), git2::Error> {
            unimplemented!()
        }

        fn checkout_branch(&self, _branch_name: &str) -> Result<(), git2::Error> {
            unimplemented!()
        }

        fn add(&self) -> Result<(), git2::Error> {
            unimplemented!()
        }

        fn commit(&self, _subject: String) -> Result<Oid, git2::Error> {
            unimplemented!()
        }

        fn push(&self, _branch_name: &str) -> Result<(), git2::Error> {
            unimplemented!()
        }
    }

    struct DefaultMockProgramOpener;

    impl ProgramOpener for DefaultMockProgramOpener {
        fn open_editor(&self, _file_path: &str) -> io::Result<()> {
            unimplemented!()
        }

        fn open_pager(&self, _file_path: &str) -> io::Result<()> {
            unimplemented!()
        }
    }
}
