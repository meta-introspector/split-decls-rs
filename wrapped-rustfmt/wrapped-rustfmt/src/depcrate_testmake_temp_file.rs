// Generated macro for make_temp_file (function)
macro_rules! Depcrate_testmake_temp_file {
() => {
// Module: crate::test
// Provides: {"make_temp_file"}
// Dependencies: {}
fn make_temp_file (file_name : & 'static str) -> TempFile { use std :: env :: var ; use std :: fs :: File ; let target_dir = var ("RUSTFMT_TEST_DIR") . unwrap_or_else (| _ | "." . to_owned ()) ; let path = Path :: new (& target_dir) . join (file_name) ; let mut file = File :: create (& path) . expect ("couldn't create temp file") ; let content = "fn main() {}\n" ; file . write_all (content . as_bytes ()) . expect ("couldn't write temp file") ; TempFile { path } }
};
}
