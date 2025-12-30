// Generated macro for write_win (function)
macro_rules! Depcratewrite_win {
() => {
// Module: crate
// Provides: {"write_win"}
// Dependencies: {}
fn write_win (output : & Path , filter : & [& str]) { let output : & str = output . as_os_str () . to_str () . unwrap () ; let mut args = vec ! ["--no-deps" , "--out" , output , "--filter"] ; args . extend_from_slice (filter) ; args . extend_from_slice (& ["--no-comment"]) ; args . extend_from_slice (& ["--flat"]) ; println ! ("running: bindgen {}" , args . join (" ")) ; _ = windows_bindgen :: bindgen (args) ; }
};
}
