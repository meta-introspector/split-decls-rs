// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl Render for InterfaceFunc { fn render (& self , src : & mut String) { rustdoc (& self . docs , src) ; if self . name . as_str () != self . name . as_str () . to_snake_case () { src . push_str ("#[link_name = \"") ; src . push_str (self . name . as_str ()) ; src . push_str ("\"]\n") ; } src . push_str ("pub fn ") ; let mut name = String :: new () ; self . name . render (& mut name) ; src . push_str (to_rust_ident (& name . to_snake_case ())) ; let (params , results) = self . wasm_signature () ; assert ! (results . len () <= 1) ; src . push_str ("(") ; for (i , param) in params . iter () . enumerate () { src . push_str (& format ! ("arg{}: " , i)) ; param . render (src) ; src . push_str (",") ; } src . push_str (")") ; if self . noreturn { src . push_str (" -> !") ; } else if let Some (result) = results . get (0) { src . push_str (" -> ") ; result . render (src) ; } src . push_str (";") ; } }
};
}
