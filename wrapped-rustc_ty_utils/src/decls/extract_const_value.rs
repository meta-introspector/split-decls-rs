macro_rules! extract_const_value {
    () => {
        fn extract_const_value < 'tcx > (cx : & LayoutCx < 'tcx > , ty : Ty < 'tcx > , ct : ty :: Const < 'tcx > ,) -> Result < ty :: Value < 'tcx > , & 'tcx LayoutError < 'tcx > > { match ct . kind () { ty :: ConstKind :: Value (cv) => Ok (cv) , ty :: ConstKind :: Param (_) | ty :: ConstKind :: Expr (_) => { if ! ct . has_param () { bug ! ("failed to normalize const, but it is not generic: {ct:?}") ; } Err (error (cx , LayoutError :: TooGeneric (ty))) } ty :: ConstKind :: Unevaluated (_) => { let err = if ct . has_param () { LayoutError :: TooGeneric (ty) } else { LayoutError :: Unknown (ty) } ; Err (error (cx , err)) } ty :: ConstKind :: Infer (_) | ty :: ConstKind :: Bound (..) | ty :: ConstKind :: Placeholder (_) | ty :: ConstKind :: Error (_) => { bug ! ("layout_of: unexpected const: {ct:?}") ; } } }
    };
}

extract_const_value!();