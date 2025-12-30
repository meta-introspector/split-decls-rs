// Generated macro for impl_26 (impl)
macro_rules! Depcrate_deriveimpl_26 {
() => {
// Module: crate::derive
// Provides: {"impl_26"}
// Dependencies: {}
impl Parse for Level { fn parse (input : ParseStream) -> syn :: Result < Self > { let ident = input . parse :: < syn :: Ident > () ? ; match ident . to_string () . as_str () { "trace" => Ok (Level :: Trace) , "debug" => Ok (Level :: Debug) , "info" => Ok (Level :: Info) , "warn" => Ok (Level :: Warn) , "error" => Ok (Level :: Error) , value => { let message = format ! ("invalid level: {}" , value) ; Err (syn :: Error :: new_spanned (ident , message)) } } } }
};
}
