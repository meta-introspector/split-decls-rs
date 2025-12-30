// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl Render for Module { fn render (& self , src : & mut String) { for f in self . funcs () { render_highlevel (& f , & self . name , src) ; src . push_str ("\n\n") ; } let rust_name = self . name . as_str () . to_snake_case () ; src . push_str ("pub mod ") ; src . push_str (& rust_name) ; src . push_str ("{\n") ; src . push_str ("#[link(wasm_import_module =\"") ; src . push_str (self . name . as_str ()) ; src . push_str ("\")]\n") ; src . push_str ("extern \"C\" {\n") ; for f in self . funcs () { f . render (src) ; src . push_str ("\n") ; } src . push_str ("}") ; src . push_str ("}") ; } }
};
}
