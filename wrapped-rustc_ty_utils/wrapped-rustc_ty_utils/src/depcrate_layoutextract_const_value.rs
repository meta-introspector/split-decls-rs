// Generated macro for extract_const_value (function)
macro_rules! Depcrate_layoutextract_const_value {
() => {
// Module: crate::layout
// Provides: {"extract_const_value"}
// Dependencies: {}
fn extract_const_value < 'tcx > (cx : & LayoutCx < 'tcx > , ty : Ty < 'tcx > , ct : ty :: Const < 'tcx > ,) -> Result < ty :: Value < 'tcx > , & 'tcx LayoutError < 'tcx > > { match ct . kind () { ty :: ConstKind :: Value (cv) => Ok (cv) , ty :: ConstKind :: Param (_) | ty :: ConstKind :: Expr (_) => { if ! ct . has_param () { bug ! ("failed to normalize const, but it is not generic: {ct:?}") ; } Err (error (cx , LayoutError :: TooGeneric (ty))) } ty :: ConstKind :: Unevaluated (_) => { let err = if ct . has_param () { LayoutError :: TooGeneric (ty) } else { LayoutError :: Unknown (ty) } ; Err (error (cx , err)) } ty :: ConstKind :: Infer (_) | ty :: ConstKind :: Bound (..) | ty :: ConstKind :: Placeholder (_) | ty :: ConstKind :: Error (_) => { bug ! ("layout_of: unexpected const: {ct:?}") ; } } }
};
}
