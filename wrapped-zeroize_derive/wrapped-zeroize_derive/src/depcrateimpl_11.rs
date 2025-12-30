// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl Parse for Bounds { fn parse (input : ParseStream < '_ >) -> Result < Self > { Ok (Self (Punctuated :: parse_terminated (input) ?)) } }
};
}
