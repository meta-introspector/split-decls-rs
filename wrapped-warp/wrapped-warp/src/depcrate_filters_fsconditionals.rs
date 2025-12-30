// Generated macro for conditionals (function)
macro_rules! Depcrate_filters_fsconditionals {
() => {
// Module: crate::filters::fs
// Provides: {"conditionals"}
// Dependencies: {}
fn conditionals () -> impl Filter < Extract = One < Conditionals > , Error = Infallible > + Copy { crate :: header :: optional2 () . and (crate :: header :: optional2 ()) . and (crate :: header :: optional2 ()) . and (crate :: header :: optional2 ()) . map (| if_modified_since , if_unmodified_since , if_range , range | Conditionals { if_modified_since , if_unmodified_since , if_range , range , } ,) }
};
}
