// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl Parse for Args { fn parse (input : ParseStream) -> Result < Self > { let vars = Punctuated :: < Ident , Token ! [,] > :: parse_terminated (input) ? ; Ok (Args { vars : vars . into_iter () . collect () , }) } }
};
}
