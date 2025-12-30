// Generated macro for format_tab_empty (function)
macro_rules! Depcrate_testsformat_tab_empty {
() => {
// Module: crate::tests
// Provides: {"format_tab_empty"}
// Dependencies: {}
# [test] fn format_tab_empty () { let fmt_pre = r###""\t{}""### ; let fmt = "\t{}" ; let parser = Parser :: new (fmt , None , Some (fmt_pre . into ()) , false , ParseMode :: Format) ; assert_eq ! (parser . collect ::< Vec < Piece <'static >>> () , & [Lit ("\t") , NextArgument (Box :: new (Argument { position : ArgumentImplicitlyIs (0) , position_span : 4 .. 4 , format : fmtdflt () , }))] ,) ; }
};
}
