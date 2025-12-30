// Generated macro for original_sp (function)
macro_rules! Depcrate_source_maporiginal_sp {
() => {
// Module: crate::source_map
// Provides: {"original_sp"}
// Dependencies: {}
# [doc = " Returns the span itself if it doesn't come from a macro expansion,"] # [doc = " otherwise return the call site span up to the `enclosing_sp` by"] # [doc = " following the `expn_data` chain."] pub fn original_sp (sp : Span , enclosing_sp : Span) -> Span { let ctxt = sp . ctxt () ; if ctxt . is_root () { return sp ; } let enclosing_ctxt = enclosing_sp . ctxt () ; let expn_data1 = ctxt . outer_expn_data () ; if ! enclosing_ctxt . is_root () && expn_data1 . call_site == enclosing_ctxt . outer_expn_data () . call_site { sp } else { original_sp (expn_data1 . call_site , enclosing_sp) } }
};
}
