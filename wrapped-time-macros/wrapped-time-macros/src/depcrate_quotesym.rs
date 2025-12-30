// Generated macro for sym (macro)
macro_rules! Depcrate_quotesym {
() => {
// Module: crate::quote
// Provides: {"sym"}
// Dependencies: {}
macro_rules ! sym { ($ ts : ident $ x : tt $ y : tt) => { $ ts . extend ([TokenTree :: from (Punct :: new ($ x , Spacing :: Joint)) , TokenTree :: from (Punct :: new ($ y , Spacing :: Alone)) ,]) ; } ; ($ ts : ident $ x : tt) => { $ ts . extend ([TokenTree :: from (Punct :: new ($ x , Spacing :: Alone))]) ; } ; }
};
}
