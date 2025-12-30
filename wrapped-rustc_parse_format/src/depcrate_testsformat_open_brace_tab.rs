// Generated macro for format_open_brace_tab (function)
macro_rules! Depcrate_testsformat_open_brace_tab {
() => {
// Module: crate::tests
// Provides: {"format_open_brace_tab"}
// Dependencies: {}
# [test] fn format_open_brace_tab () { let fmt_pre = r###""{\t""### ; let fmt = "{\t" ; let mut parser = Parser :: new (fmt , None , Some (fmt_pre . into ()) , false , ParseMode :: Format) ; let _ = parser . by_ref () . collect :: < Vec < Piece < 'static > > > () ; assert_eq ! (parser . errors [0] . span , 4 .. 4) ; }
};
}
