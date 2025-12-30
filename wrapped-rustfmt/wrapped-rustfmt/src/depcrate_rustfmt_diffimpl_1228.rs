// Generated macro for impl_1228 (impl)
macro_rules! Depcrate_rustfmt_diffimpl_1228 {
() => {
// Module: crate::rustfmt_diff
// Provides: {"impl_1228"}
// Dependencies: {}
impl OutputWriter { pub (crate) fn new (color : Color) -> Self { if let Some (t) = term :: stdout () { if color . use_colored_tty () && t . supports_color () { return OutputWriter { terminal : Some (t) } ; } } OutputWriter { terminal : None } } pub (crate) fn writeln (& mut self , msg : & str , color : Option < term :: color :: Color >) { match & mut self . terminal { Some (ref mut t) => { if let Some (color) = color { t . fg (color) . unwrap () ; } writeln ! (t , "{msg}") . unwrap () ; if color . is_some () { t . reset () . unwrap () ; } } None => println ! ("{msg}") , } } }
};
}
