// Generated macro for render_highlevel (function)
macro_rules! Depcraterender_highlevel {
() => {
// Module: crate
// Provides: {"render_highlevel"}
// Dependencies: {}
fn render_highlevel (func : & InterfaceFunc , module : & Id , src : & mut String) { let mut rust_name = String :: new () ; func . name . render (& mut rust_name) ; let rust_name = rust_name . to_snake_case () ; rustdoc (& func . docs , src) ; rustdoc_params (& func . params , "Parameters" , src) ; rustdoc_params (& func . results , "Return" , src) ; src . push_str ("pub unsafe fn ") ; if cfg ! (feature = "multi-module") { src . push_str (& [module . as_str () . to_snake_case () . as_str () , & rust_name] . join ("_")) ; } else { src . push_str (to_rust_ident (& rust_name)) ; } src . push_str ("(") ; for param in func . params . iter () { param . name . render (src) ; src . push_str (": ") ; param . tref . render (src) ; src . push_str (",") ; } src . push_str (")") ; match func . results . len () { 0 => { } 1 => { src . push_str (" -> ") ; func . results [0] . tref . render (src) ; } _ => { src . push_str (" -> (") ; for result in func . results . iter () { result . tref . render (src) ; src . push_str (", ") ; } src . push_str (")") ; } } src . push_str ("{") ; func . call_wasm (module , & mut Rust { src , params : & func . params , block_storage : Vec :: new () , blocks : Vec :: new () , } ,) ; src . push_str ("}") ; }
};
}
