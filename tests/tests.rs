#[cfg(test)]
mod tests {
    use eureka::config_manager::{ConfigManagement, ConfigType};
    use eureka::printer::{Print, PrintColor, PrintOptions};
    use eureka::reader::ReadInput;
    use eureka::{Eureka, EurekaOptions};

    use eureka::git::GitManagement;
    use eureka::program_access::ProgramOpener;
    use git2::Oid;
    use std::io;
    use std::io::{Error, ErrorKind};
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_clear_repo() {
        struct MockConfigManager;
        static READ_COUNTER: AtomicUsize = AtomicUsize::new(0);

        impl ConfigManagement for MockConfigManager {
            fn config_dir_create(&self) -> io::Result<()> {
                unimplemented!()
            }

            fn config_dir_exists(&self) -> bool {
                unimplemented!()
            }

            fn config_read(&self, file: ConfigType) -> io::Result<String> {
                let counter = READ_COUNTER.fetch_add(1, Ordering::SeqCst);
                if counter > 0 {
                    panic!("Should only be read once");
                }

                assert_eq!(file, ConfigType::Repo);
                Ok("some-path".to_string())
            }

            fn config_write(&self, _file: ConfigType, _value: String) -> io::Result<()> {
                unimplemented!()
            }

            fn config_rm(&self, config_type: ConfigType) -> io::Result<()> {
                assert_eq!(config_type, ConfigType::Repo);
                Ok(())
            }
        }

        let mut eureka = Eureka::new(
            MockConfigManager {},
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
        struct MockConfigManager;

        impl ConfigManagement for MockConfigManager {
            fn config_dir_create(&self) -> io::Result<()> {
                unimplemented!()
            }

            fn config_dir_exists(&self) -> bool {
                unimplemented!()
            }

            fn config_read(&self, file: ConfigType) -> io::Result<String> {
                assert_eq!(file, ConfigType::Branch);
                Ok("some-path".to_string())
            }

            fn config_write(&self, _file: ConfigType, _value: String) -> io::Result<()> {
                unimplemented!()
            }

            fn config_rm(&self, config_type: ConfigType) -> io::Result<()> {
                assert_eq!(config_type, ConfigType::Branch);
                Ok(())
            }
        }

        let mut eureka = Eureka::new(
            MockConfigManager {},
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
        struct MockConfigManager;
        static READ_COUNTER: AtomicUsize = AtomicUsize::new(0);
        static RM_COUNTER: AtomicUsize = AtomicUsize::new(0);

        impl ConfigManagement for MockConfigManager {
            fn config_dir_create(&self) -> io::Result<()> {
                unimplemented!()
            }

            fn config_dir_exists(&self) -> bool {
                unimplemented!()
            }

            fn config_read(&self, file: ConfigType) -> io::Result<String> {
                let counter = READ_COUNTER.fetch_add(1, Ordering::SeqCst);
                if counter == 0 {
                    assert_eq!(file, ConfigType::Repo);
                } else if counter == 1 {
                    assert_eq!(file, ConfigType::Branch);
                } else {
                    panic!("Should not be read this many times")
                }

                Ok("some-path".to_string())
            }

            fn config_write(&self, _file: ConfigType, _value: String) -> io::Result<()> {
                unimplemented!()
            }

            fn config_rm(&self, config_type: ConfigType) -> io::Result<()> {
                let counter = RM_COUNTER.fetch_add(1, Ordering::SeqCst);
                if counter == 0 {
                    assert_eq!(config_type, ConfigType::Repo);
                } else if counter == 1 {
                    assert_eq!(config_type, ConfigType::Branch);
                }

                Ok(())
            }
        }

        let mut eureka = Eureka::new(
            MockConfigManager {},
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
        struct MockConfigManager;
        static READ_COUNTER: AtomicUsize = AtomicUsize::new(0);

        impl ConfigManagement for MockConfigManager {
            fn config_dir_create(&self) -> io::Result<()> {
                unimplemented!()
            }

            fn config_dir_exists(&self) -> bool {
                unimplemented!()
            }

            fn config_read(&self, file: ConfigType) -> io::Result<String> {
                let counter = READ_COUNTER.fetch_add(1, Ordering::SeqCst);
                if counter > 0 {
                    panic!("Should only be read once");
                }

                assert_eq!(file, ConfigType::Repo);
                Ok("specific-repo-path".to_string())
            }

            fn config_write(&self, _file: ConfigType, _value: String) -> io::Result<()> {
                unimplemented!()
            }

            fn config_rm(&self, _config_type: ConfigType) -> io::Result<()> {
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
            MockConfigManager,
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
        struct MockConfigManager;
        static READ_COUNTER: AtomicUsize = AtomicUsize::new(0);

        impl ConfigManagement for MockConfigManager {
            fn config_dir_create(&self) -> io::Result<()> {
                Ok(())
            }

            fn config_dir_exists(&self) -> bool {
                // Config dir is missing
                false
            }

            fn config_read(&self, _file: ConfigType) -> io::Result<String> {
                let counter = READ_COUNTER.fetch_add(1, Ordering::SeqCst);
                if counter == 0 {
                    // First it checks if any config can be found and
                    // based on that it decides to create the config dir
                    Err(Error::new(ErrorKind::Other, "some-error"))
                } else {
                    Ok(String::from("some-ok"))
                }
            }

            fn config_write(&self, _file: ConfigType, _value: String) -> io::Result<()> {
                unimplemented!()
            }

            fn config_rm(&self, _config_type: ConfigType) -> io::Result<()> {
                unimplemented!()
            }
        }

        struct MockPrinter;

        impl Print for MockPrinter {
            fn print(&mut self, _value: &str) -> io::Result<()> {
                unimplemented!()
            }

            fn println(&mut self, value: &str) -> io::Result<()> {
                assert_eq!(value, "First time setup complete. Happy ideation!");
                Ok(())
            }
        }

        impl PrintColor for MockPrinter {
            fn fts_banner(&mut self) -> io::Result<()> {
                // noop
                Ok(())
            }

            fn input_header(&mut self, _value: &str) -> io::Result<()> {
                unimplemented!()
            }

            fn println_styled(&mut self, _value: &str, _opts: PrintOptions) -> io::Result<()> {
                unimplemented!()
            }
        }

        let mut eureka = Eureka::new(
            MockConfigManager {},
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
    fn test_setup_repo_and_branch() {
        static INPUT_HEADER_COUNTER: AtomicUsize = AtomicUsize::new(0);
        static READ_INPUT_COUNTER: AtomicUsize = AtomicUsize::new(0);

        struct MockConfigManager;

        impl ConfigManagement for MockConfigManager {
            fn config_dir_create(&self) -> io::Result<()> {
                Ok(())
            }

            fn config_dir_exists(&self) -> bool {
                true
            }

            fn config_read(&self, _file: ConfigType) -> io::Result<String> {
                Err(Error::new(ErrorKind::Other, "some-error"))
            }

            fn config_write(&self, file: ConfigType, value: String) -> io::Result<()> {
                match file {
                    ConfigType::Branch => assert_eq!(value, "specific-branch-name"),
                    ConfigType::Repo => assert_eq!(value, "specific-repo-path"),
                }
                Ok(())
            }

            fn config_rm(&self, _config_type: ConfigType) -> io::Result<()> {
                unimplemented!()
            }
        }

        struct MockPrinter;

        impl Print for MockPrinter {
            fn print(&mut self, _value: &str) -> io::Result<()> {
                unimplemented!()
            }

            fn println(&mut self, value: &str) -> io::Result<()> {
                assert_eq!(value, "First time setup complete. Happy ideation!");
                Ok(())
            }
        }

        impl PrintColor for MockPrinter {
            fn fts_banner(&mut self) -> io::Result<()> {
                // noop
                Ok(())
            }

            fn input_header(&mut self, value: &str) -> io::Result<()> {
                let counter = INPUT_HEADER_COUNTER.fetch_add(1, Ordering::SeqCst);
                if counter == 0 {
                    assert_eq!(value, "Absolute path to your idea repo");
                } else {
                    assert_eq!(value, "Name of branch (default: master)");
                }

                Ok(())
            }

            fn println_styled(&mut self, _value: &str, _opts: PrintOptions) -> io::Result<()> {
                unimplemented!()
            }
        }

        struct MockReader;

        impl ReadInput for MockReader {
            fn read_input(&mut self) -> io::Result<String> {
                let counter = READ_INPUT_COUNTER.fetch_add(1, Ordering::SeqCst);
                if counter == 0 {
                    // Repo
                    Ok(String::from("specific-repo-path"))
                } else {
                    // Branch
                    Ok(String::from("specific-branch-name"))
                }
            }
        }

        let mut eureka = Eureka::new(
            MockConfigManager {},
            MockPrinter {},
            MockReader {},
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
    }

    #[test]
    fn test_setup_defaults_to_master_branch() {
        static INPUT_HEADER_COUNTER: AtomicUsize = AtomicUsize::new(0);
        static READ_INPUT_COUNTER: AtomicUsize = AtomicUsize::new(0);

        struct MockConfigManager;

        impl ConfigManagement for MockConfigManager {
            fn config_dir_create(&self) -> io::Result<()> {
                Ok(())
            }

            fn config_dir_exists(&self) -> bool {
                true
            }

            fn config_read(&self, _file: ConfigType) -> io::Result<String> {
                Err(Error::new(ErrorKind::Other, "some-error"))
            }

            fn config_write(&self, file: ConfigType, value: String) -> io::Result<()> {
                match file {
                    ConfigType::Branch => assert_eq!(value, "master"),
                    ConfigType::Repo => assert_eq!(value, "specific-repo-path"),
                }
                Ok(())
            }

            fn config_rm(&self, _config_type: ConfigType) -> io::Result<()> {
                unimplemented!()
            }
        }

        struct MockPrinter;

        impl Print for MockPrinter {
            fn print(&mut self, _value: &str) -> io::Result<()> {
                unimplemented!()
            }

            fn println(&mut self, value: &str) -> io::Result<()> {
                assert_eq!(value, "First time setup complete. Happy ideation!");
                Ok(())
            }
        }

        impl PrintColor for MockPrinter {
            fn fts_banner(&mut self) -> io::Result<()> {
                // noop
                Ok(())
            }

            fn input_header(&mut self, value: &str) -> io::Result<()> {
                let counter = INPUT_HEADER_COUNTER.fetch_add(1, Ordering::SeqCst);
                if counter == 0 {
                    assert_eq!(value, "Absolute path to your idea repo");
                } else {
                    assert_eq!(value, "Name of branch (default: master)");
                }

                Ok(())
            }

            fn println_styled(&mut self, _value: &str, _opts: PrintOptions) -> io::Result<()> {
                unimplemented!()
            }
        }

        struct MockReader;

        impl ReadInput for MockReader {
            fn read_input(&mut self) -> io::Result<String> {
                let counter = READ_INPUT_COUNTER.fetch_add(1, Ordering::SeqCst);
                if counter == 0 {
                    // Repo
                    Ok(String::from("specific-repo-path"))
                } else {
                    // Branch
                    // Leave empty to default to "master"
                    Ok(String::new())
                }
            }
        }

        let mut eureka = Eureka::new(
            MockConfigManager {},
            MockPrinter {},
            MockReader {},
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
    }

    #[test]
    fn test_setup_repo_path_asks_until_user_provides_value() {
        static INPUT_HEADER_COUNTER: AtomicUsize = AtomicUsize::new(0);
        static READ_INPUT_COUNTER: AtomicUsize = AtomicUsize::new(0);

        struct MockConfigManager;

        impl ConfigManagement for MockConfigManager {
            fn config_dir_create(&self) -> io::Result<()> {
                Ok(())
            }

            fn config_dir_exists(&self) -> bool {
                true
            }

            fn config_read(&self, _file: ConfigType) -> io::Result<String> {
                Err(Error::new(ErrorKind::Other, "some-error"))
            }

            fn config_write(&self, file: ConfigType, value: String) -> io::Result<()> {
                match file {
                    ConfigType::Branch => assert_eq!(value, "specific-branch-name"),
                    ConfigType::Repo => assert_eq!(value, "specific-repo-path"),
                }
                Ok(())
            }

            fn config_rm(&self, _config_type: ConfigType) -> io::Result<()> {
                unimplemented!()
            }
        }

        struct MockPrinter;

        impl Print for MockPrinter {
            fn print(&mut self, _value: &str) -> io::Result<()> {
                unimplemented!()
            }

            fn println(&mut self, value: &str) -> io::Result<()> {
                assert_eq!(value, "First time setup complete. Happy ideation!");
                Ok(())
            }
        }

        impl PrintColor for MockPrinter {
            fn fts_banner(&mut self) -> io::Result<()> {
                // noop
                Ok(())
            }

            fn input_header(&mut self, value: &str) -> io::Result<()> {
                let counter = INPUT_HEADER_COUNTER.fetch_add(1, Ordering::SeqCst);
                if counter <= 5 {
                    assert_eq!(value, "Absolute path to your idea repo");
                } else {
                    assert_eq!(value, "Name of branch (default: master)");
                }
                Ok(())
            }

            fn println_styled(&mut self, _value: &str, _opts: PrintOptions) -> io::Result<()> {
                unimplemented!()
            }
        }

        struct MockReader;

        impl ReadInput for MockReader {
            fn read_input(&mut self) -> io::Result<String> {
                let counter = READ_INPUT_COUNTER.fetch_add(1, Ordering::SeqCst);
                if counter < 5 {
                    // Return empty string to prompt it to ask again
                    Ok(String::new())
                } else if counter == 5 {
                    // Repo
                    Ok(String::from("specific-repo-path"))
                } else {
                    // Branch
                    Ok(String::from("specific-branch-name"))
                }
            }
        }

        let mut eureka = Eureka::new(
            MockConfigManager {},
            MockPrinter {},
            MockReader {},
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
    }

    #[test]
    fn test_idea_summary_asks_until_user_provides_value() {
        static INPUT_HEADER_COUNTER: AtomicUsize = AtomicUsize::new(0);
        static READ_INPUT_COUNTER: AtomicUsize = AtomicUsize::new(0);

        struct MockConfigManager;

        impl ConfigManagement for MockConfigManager {
            fn config_dir_create(&self) -> io::Result<()> {
                Ok(())
            }

            fn config_dir_exists(&self) -> bool {
                true
            }

            fn config_read(&self, _file: ConfigType) -> io::Result<String> {
                Ok(String::from("specific-config-string"))
            }

            fn config_write(&self, file: ConfigType, value: String) -> io::Result<()> {
                match file {
                    ConfigType::Branch => assert_eq!(value, "specific-branch-name"),
                    ConfigType::Repo => assert_eq!(value, "specific-repo-path"),
                }
                Ok(())
            }

            fn config_rm(&self, _config_type: ConfigType) -> io::Result<()> {
                unimplemented!()
            }
        }

        struct MockPrinter;

        impl Print for MockPrinter {
            fn print(&mut self, value: &str) -> io::Result<()> {
                assert_eq!(value, "First time setup complete. Happy ideation!");
                Ok(())
            }

            fn println(&mut self, _value: &str) -> io::Result<()> {
                // noop
                Ok(())
            }
        }

        impl PrintColor for MockPrinter {
            fn fts_banner(&mut self) -> io::Result<()> {
                // noop
                Ok(())
            }

            fn input_header(&mut self, value: &str) -> io::Result<()> {
                let counter = INPUT_HEADER_COUNTER.fetch_add(1, Ordering::SeqCst);
                if counter <= 5 {
                    assert_eq!(value, ">> Idea summary");
                } else {
                    assert_eq!(value, "Name of branch (default: master)");
                }
                Ok(())
            }

            fn println_styled(&mut self, _value: &str, _opts: PrintOptions) -> io::Result<()> {
                unimplemented!()
            }
        }

        struct MockReader;

        impl ReadInput for MockReader {
            fn read_input(&mut self) -> io::Result<String> {
                let counter = READ_INPUT_COUNTER.fetch_add(1, Ordering::SeqCst);
                if counter < 5 {
                    // Return empty string to prompt it to ask again
                    Ok(String::new())
                } else if counter == 5 {
                    Ok(String::from("specific-idea-summary"))
                } else {
                    unimplemented!()
                }
            }
        }

        struct MockGit;

        impl GitManagement for MockGit {
            fn init(&mut self, _repo_path: &str) -> Result<(), git2::Error> {
                Ok(())
            }

            fn checkout_branch(&self, _branch_name: &str) -> Result<(), git2::Error> {
                Ok(())
            }

            fn add(&self) -> Result<(), git2::Error> {
                Ok(())
            }

            fn commit(&self, _subject: &str) -> Result<Oid, git2::Error> {
                Ok(Oid::zero())
            }

            fn push(&self, _branch_name: &str) -> Result<(), git2::Error> {
                Ok(())
            }
        }

        struct MockProgramAccess;

        impl ProgramOpener for MockProgramAccess {
            fn open_editor(&self, _file_path: &str) -> io::Result<()> {
                Ok(())
            }

            fn open_pager(&self, _file_path: &str) -> io::Result<()> {
                Ok(())
            }
        }

        let mut eureka = Eureka::new(
            MockConfigManager {},
            MockPrinter {},
            MockReader {},
            MockGit {},
            MockProgramAccess {},
        );
        let opts = EurekaOptions {
            clear_repo: false,
            clear_branch: false,
            view: false,
        };

        let actual = eureka.run(opts);

        assert!(actual.is_ok());
    }

    #[test]
    fn test_e2e_happy_path() {
        static PRINT_COUNTER: AtomicUsize = AtomicUsize::new(0);

        struct MockConfigManager;

        impl ConfigManagement for MockConfigManager {
            fn config_dir_create(&self) -> io::Result<()> {
                unimplemented!()
            }

            fn config_dir_exists(&self) -> bool {
                true
            }

            fn config_read(&self, file: ConfigType) -> io::Result<String> {
                match file {
                    ConfigType::Branch => Ok("specific-branch".to_string()),
                    ConfigType::Repo => Ok("specific-repo".to_string()),
                }
            }

            fn config_write(&self, _file: ConfigType, _value: String) -> io::Result<()> {
                unimplemented!()
            }

            fn config_rm(&self, _config_type: ConfigType) -> io::Result<()> {
                unimplemented!()
            }
        }

        struct MockPrinter;

        impl Print for MockPrinter {
            fn print(&mut self, _value: &str) -> io::Result<()> {
                unimplemented!()
            }

            fn println(&mut self, value: &str) -> io::Result<()> {
                let counter = PRINT_COUNTER.fetch_add(1, Ordering::SeqCst);
                match counter {
                    0 => assert_eq!(
                        value,
                        "Adding and committing your new idea to specific-branch.."
                    ),
                    1 => assert_eq!(value, "Added and committed!"),
                    2 => assert_eq!(value, "Pushing your new idea.."),
                    3 => assert_eq!(value, "Pushed!"),
                    _ => panic!("Unknown state"),
                }

                Ok(())
            }
        }

        impl PrintColor for MockPrinter {
            fn fts_banner(&mut self) -> io::Result<()> {
                unimplemented!()
            }

            fn input_header(&mut self, value: &str) -> io::Result<()> {
                assert_eq!(value, ">> Idea summary");
                Ok(())
            }

            fn println_styled(&mut self, _value: &str, _opts: PrintOptions) -> io::Result<()> {
                unimplemented!()
            }
        }

        struct MockReader;

        impl ReadInput for MockReader {
            fn read_input(&mut self) -> io::Result<String> {
                Ok(String::from("read-input-string"))
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

            fn commit(&self, subject: &str) -> Result<Oid, git2::Error> {
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
            MockConfigManager {},
            MockPrinter {},
            MockReader {},
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
        fn print(&mut self, _value: &str) -> io::Result<()> {
            unimplemented!()
        }

        fn println(&mut self, _value: &str) -> io::Result<()> {
            unimplemented!()
        }
    }

    impl PrintColor for DefaultMockPrinter {
        fn fts_banner(&mut self) -> io::Result<()> {
            unimplemented!()
        }

        fn input_header(&mut self, _value: &str) -> io::Result<()> {
            unimplemented!()
        }

        fn println_styled(&mut self, _value: &str, _opts: PrintOptions) -> io::Result<()> {
            unimplemented!()
        }
    }

    struct DefaultMockReader;

    impl ReadInput for DefaultMockReader {
        fn read_input(&mut self) -> io::Result<String> {
            unimplemented!()
        }
    }

    struct DefaultMockConfigManager;

    impl ConfigManagement for DefaultMockConfigManager {
        fn config_dir_create(&self) -> io::Result<()> {
            unimplemented!()
        }

        fn config_dir_exists(&self) -> bool {
            unimplemented!()
        }

        fn config_read(&self, _file: ConfigType) -> io::Result<String> {
            unimplemented!()
        }

        fn config_write(&self, _file: ConfigType, _value: String) -> io::Result<()> {
            unimplemented!()
        }

        fn config_rm(&self, _config_type: ConfigType) -> io::Result<()> {
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

        fn commit(&self, _subject: &str) -> Result<Oid, git2::Error> {
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
