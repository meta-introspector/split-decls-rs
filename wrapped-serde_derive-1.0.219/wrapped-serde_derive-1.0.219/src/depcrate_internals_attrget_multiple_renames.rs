// Generated macro for get_multiple_renames (function)
macro_rules! Depcrate_internals_attrget_multiple_renames {
() => {
// Module: crate::internals::attr
// Provides: {"get_multiple_renames"}
// Dependencies: {}
fn get_multiple_renames (cx : & Ctxt , meta : & ParseNestedMeta ,) -> syn :: Result < (Option < syn :: LitStr > , Vec < syn :: LitStr >) > { let (ser , de) = get_ser_and_de (cx , RENAME , meta , get_lit_str2) ? ; Ok ((ser . at_most_one () , de . get ())) }
};
}
