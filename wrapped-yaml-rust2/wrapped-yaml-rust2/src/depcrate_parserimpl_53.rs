// Generated macro for impl_53 (impl)
macro_rules! Depcrate_parserimpl_53 {
() => {
// Module: crate::parser
// Provides: {"impl_53"}
// Dependencies: {}
impl < 'a > Parser < core :: str :: Chars < 'a > > { # [doc = " Create a new instance of a parser from a &str."] # [must_use] pub fn new_from_str (value : & 'a str) -> Self { Parser :: new (value . chars ()) } }
};
}
