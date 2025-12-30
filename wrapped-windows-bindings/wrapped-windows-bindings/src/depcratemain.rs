// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let out_file = "../src/wrapper/windows_sys.rs" ; windows_bindgen :: bindgen (& ["--out" , out_file , "--etc" , "./bindings.config"]) ; let mut bindings = std :: fs :: File :: options () . append (true) . open (out_file) . unwrap () ; bindings . write_all (LINKAGE) . unwrap () ; }
};
}
