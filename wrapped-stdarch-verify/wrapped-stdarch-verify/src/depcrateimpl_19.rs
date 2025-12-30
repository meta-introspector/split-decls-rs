// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
impl syn :: parse :: Parse for RustcArgsRequiredConst { fn parse (input : syn :: parse :: ParseStream < '_ >) -> syn :: Result < Self > { let list = syn :: punctuated :: Punctuated :: < syn :: LitInt , Token ! [,] > :: parse_terminated (input) ? ; Ok (Self { args : list . into_iter () . map (| a | a . base10_parse :: < usize > ()) . collect :: < syn :: Result < _ > > () ? , }) } }
};
}
