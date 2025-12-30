// Generated macro for impl_432 (impl)
macro_rules! Depcrate_layout_testimpl_432 {
() => {
// Module: crate::layout_test
// Provides: {"impl_432"}
// Dependencies: {}
impl < 'tcx > LayoutOfHelpers < 'tcx > for UnwrapLayoutCx < 'tcx > { fn handle_layout_err (& self , err : LayoutError < 'tcx > , span : Span , ty : Ty < 'tcx >) -> ! { span_bug ! (span , "`#[rustc_layout(..)]` test resulted in `layout_of({ty}) = Err({err})`" ,) ; } }
};
}
