// Generated macro for bindgen (function)
macro_rules! Depcratebindgen {
() => {
// Module: crate
// Provides: {"bindgen"}
// Dependencies: {}
fn bindgen (output : & Path , filter : & [& str] , config : & [& str]) { let output : & str = output . as_os_str () . to_str () . unwrap () ; let mut args = vec ! ["--no-deps" , "--out" , output , "--filter"] ; args . extend_from_slice (filter) ; args . extend_from_slice (& ["--no-comment"]) ; args . extend_from_slice (config) ; println ! ("running: bindgen {}" , args . join (" ")) ; _ = windows_bindgen :: bindgen (args) ; }
};
}
