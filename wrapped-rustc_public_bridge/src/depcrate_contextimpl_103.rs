// Generated macro for impl_103 (impl)
macro_rules! Depcrate_contextimpl_103 {
() => {
// Module: crate::context
// Provides: {"impl_103"}
// Dependencies: {}
impl < 'tcx , B : Bridge > LayoutOfHelpers < 'tcx > for CompilerCtxt < 'tcx , B > { type LayoutOfResult = Result < ty :: layout :: TyAndLayout < 'tcx > , B :: Error > ; # [inline] fn handle_layout_err (& self , err : ty :: layout :: LayoutError < 'tcx > , _span : rustc_span :: Span , ty : Ty < 'tcx > ,) -> B :: Error { B :: Error :: new (format ! ("Failed to get layout for `{ty}`: {err}")) } }
};
}
