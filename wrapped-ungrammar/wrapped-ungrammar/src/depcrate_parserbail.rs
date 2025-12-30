// Generated macro for bail (macro)
macro_rules! Depcrate_parserbail {
() => {
// Module: crate::parser
// Provides: {"bail"}
// Dependencies: {}
macro_rules ! bail { ($ loc : expr , $ ($ tt : tt) *) => { { let err = $ crate :: error :: format_err ! ($ ($ tt) *) . with_location ($ loc) ; return Err (err) ; } } ; }
};
}
