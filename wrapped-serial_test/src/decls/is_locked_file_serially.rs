macro_rules! deps {
    () => {
        Lock!();
    };
}

macro_rules! is_locked_file_serially {
    () => {
        deps!();
        # [doc = " Check if the current thread is holding a `file_serial` lock"] # [doc = ""] # [doc = " Can be used to assert that a piece of code can only be called"] # [doc = " from a test marked `#[file_serial]`."] # [doc = ""] # [doc = " Example, with `#[file_serial]`:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use serial_test::{is_locked_file_serially, file_serial};"] # [doc = ""] # [doc = " fn do_something_in_need_of_serialization() {"] # [doc = "     assert!(is_locked_file_serially(None, None));"] # [doc = ""] # [doc = "     // ..."] # [doc = " }"] # [doc = ""] # [doc = " #[test]"] # [doc = " # fn unused() {}"] # [doc = " #[file_serial]"] # [doc = " fn main() {"] # [doc = "     do_something_in_need_of_serialization();"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Example, missing `#[file_serial]`:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use serial_test::{is_locked_file_serially, file_serial};"] # [doc = ""] # [doc = " #[test]"] # [doc = " # fn unused() {}"] # [doc = " // #[file_serial] // <-- missing"] # [doc = " fn main() {"] # [doc = "     assert!(is_locked_file_serially(None, None));"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Example, `#[test(some_key)]`:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use serial_test::{is_locked_file_serially, file_serial};"] # [doc = ""] # [doc = " #[test]"] # [doc = " # fn unused() {}"] # [doc = " #[file_serial(some_key)]"] # [doc = " fn main() {"] # [doc = "     assert!(is_locked_file_serially(Some(\"some_key\"), None));"] # [doc = "     assert!(!is_locked_file_serially(None, None));"] # [doc = " }"] # [doc = " ```"] pub fn is_locked_file_serially (name : Option < & str > , path : Option < & str >) -> bool { if let Some (opt_path) = path { Lock :: is_locked (opt_path) } else { let default_path = path_for_name (name . unwrap_or_default ()) ; Lock :: is_locked (& default_path) } }
    };
}

is_locked_file_serially!();