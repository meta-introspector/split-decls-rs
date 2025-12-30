// Generated macro for impl_30 (impl)
macro_rules! Depcrateimpl_30 {
() => {
// Module: crate
// Provides: {"impl_30"}
// Dependencies: {}
impl IdentIsRaw { pub fn yes (self) -> bool { matches ! (self , IdentIsRaw :: Yes) } pub fn no (& self) -> bool { matches ! (self , IdentIsRaw :: No) } pub fn as_str (self) -> & 'static str { match self { IdentIsRaw :: No => "" , IdentIsRaw :: Yes => "r#" , } } pub fn split_from_symbol (sym : & str) -> (Self , & str) { if let Some (sym) = sym . strip_prefix ("r#") { (IdentIsRaw :: Yes , sym) } else { (IdentIsRaw :: No , sym) } } }
};
}
