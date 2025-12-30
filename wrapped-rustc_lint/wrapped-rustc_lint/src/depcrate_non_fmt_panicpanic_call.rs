// Generated macro for panic_call (function)
macro_rules! Depcrate_non_fmt_panicpanic_call {
() => {
// Module: crate::non_fmt_panic
// Provides: {"panic_call"}
// Dependencies: {}
fn panic_call < 'tcx > (cx : & LateContext < 'tcx > , f : & 'tcx hir :: Expr < 'tcx > ,) -> (Span , Option < Symbol > , Symbol) { let mut expn = f . span . ctxt () . outer_expn_data () ; let mut panic_macro = None ; loop { let parent = expn . call_site . ctxt () . outer_expn_data () ; let Some (id) = parent . macro_def_id else { break } ; let Some (name) = cx . tcx . get_diagnostic_name (id) else { break } ; if ! matches ! (name , sym :: core_panic_macro | sym :: std_panic_macro | sym :: assert_macro | sym :: debug_assert_macro | sym :: unreachable_macro) { break ; } expn = parent ; panic_macro = Some (name) ; } let macro_symbol = if let hygiene :: ExpnKind :: Macro (_ , symbol) = expn . kind { symbol } else { sym :: panic } ; (expn . call_site , panic_macro , macro_symbol) }
};
}
