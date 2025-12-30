// Generated macro for cons_tuple (function)
macro_rules! Depcrate_arbitrarycons_tuple {
() => {
// Module: crate::arbitrary
// Provides: {"cons_tuple"}
// Dependencies: {}
fn cons_tuple (es : & [impl ToTokens]) -> TokenStream { match es { [] => quote ! (()) , [e0] => quote ! (# e0) , [e0 , e1] => quote ! ((# e0 , # e1)) , [e0 , el @ ..] => { let el = cons_tuple (el) ; quote ! ((# e0 , # el)) } } }
};
}
