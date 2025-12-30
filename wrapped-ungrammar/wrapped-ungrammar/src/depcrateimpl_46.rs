// Generated macro for impl_46 (impl)
macro_rules! Depcrateimpl_46 {
() => {
// Module: crate
// Provides: {"impl_46"}
// Dependencies: {}
impl Grammar { # [doc = " Returns an iterator over all nodes in the grammar."] pub fn iter (& self) -> impl Iterator < Item = Node > + '_ { (0 .. self . nodes . len ()) . map (Node) } # [doc = " Returns an iterator over all tokens in the grammar."] pub fn tokens (& self) -> impl Iterator < Item = Token > + '_ { (0 .. self . tokens . len ()) . map (Token) } }
};
}
