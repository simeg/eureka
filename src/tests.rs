#![feature(use_extern_macros)]

#[cfg(test)] extern crate mockers_derive;
#[cfg(test)] extern crate mockers;

#[cfg(test)] use self::mockers_derive::mocked;

use self::mockers::Scenario;

use std::io::Error;
use std::io::ErrorKind;

use types::ConfigType;

use super::*;

//    #[test]
//    fn tests_work() {
//        let fh = MockFileHandler {};
//        //        let mut output = Vec::new();
//        let mut output = StandardStream::stdout(ColorChoice::AlwaysAnsi);
//        let input = b"I'm George";
//        let mut reader = Reader { reader: &input[..] };
//        let mut printer = Printer { writer: output };
//        run(&fh, &mut printer, &mut reader);
//
////        assert_eq!(true, true);
//    }

//struct MockFhConfigReadErr;
//struct MockFhConfigReadOk;
//
//impl ConfigManagement for MockFhConfigReadErr {
//    fn config_dir_create(&self) -> Result<String, Error> {
//        unimplemented!()
//    }
//
//    fn config_dir_exists(&self) -> bool {
//        unimplemented!()
//    }
//
//    fn config_read(&self, file: ConfigType) -> Result<String, Error> {
//        Err(io::Error::new(ErrorKind::NotFound, "irrelevant-error-msg"))
//    }
//
//    fn config_write(&self, file: ConfigType, value: &String) -> Result<(), Error> {
//        unimplemented!()
//    }
//}
//
//impl ConfigManagement for MockFhConfigReadOk {
//    fn config_dir_create(&self) -> Result<String, Error> {
//        unimplemented!()
//    }
//
//    fn config_dir_exists(&self) -> bool {
//        unimplemented!()
//    }
//
//    fn config_read(&self, file: ConfigType) -> Result<String, Error> {
//        Ok("irrelevant-str".to_string())
//    }
//
//    fn config_write(&self, file: ConfigType, value: &String) -> Result<(), Error> {
//        unimplemented!()
//    }
//}

#[test]
fn test_is_first_time_run() {
//    let fh_err = MockFhConfigReadErr {};
//    let fh_ok = MockFhConfigReadOk {};

    let scenario = Scenario::new();
    let mut fh = scenario.create_mock::<ConfigManagement>();

//    scenario.expect(fh.get_temperature_call().and_return(16));
//    scenario.expect(fh.make_hotter_call(4).and_return(()));

    scenario.expect(fh.config_read().and_return(Ok("irrelevant")));

//    assert_eq!(is_first_time_run(&fh), true);
    assert_eq!(is_first_time_run(&fh), false);
}
