// Generated macro for impl_181 (impl)
macro_rules! Depcrate_queryimpl_181 {
() => {
// Module: crate::query
// Provides: {"impl_181"}
// Dependencies: {}
impl < T : Parse > Parse for List < T > { fn parse (input : ParseStream < '_ >) -> Result < Self > { let mut list = Vec :: new () ; while ! input . is_empty () { list . push (input . parse () ?) ; } Ok (List (list)) } }
};
}
