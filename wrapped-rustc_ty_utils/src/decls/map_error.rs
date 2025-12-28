macro_rules! deps {
    () => {
        NonPrimitiveSimdType!();
        ZeroLengthSimdType!();
        OversizedSimdType!();
    };
}

macro_rules! map_error {
    () => {
        deps!();
        fn map_error < 'tcx > (cx : & LayoutCx < 'tcx > , ty : Ty < 'tcx > , err : LayoutCalculatorError < TyAndLayout < 'tcx > > ,) -> & 'tcx LayoutError < 'tcx > { let err = match err { LayoutCalculatorError :: SizeOverflow => { LayoutError :: SizeOverflow (ty) } LayoutCalculatorError :: UnexpectedUnsized (field) => { assert ! (field . layout . is_unsized () , "invalid layout error {err:#?}") ; if cx . typing_env . param_env . caller_bounds () . is_empty () { cx . tcx () . dcx () . delayed_bug (format ! ("encountered unexpected unsized field in layout of {ty:?}: {field:#?}")) ; } LayoutError :: Unknown (ty) } LayoutCalculatorError :: EmptyUnion => { let guar = cx . tcx () . dcx () . delayed_bug (format ! ("computed layout of empty union: {ty:?}")) ; LayoutError :: ReferencesError (guar) } LayoutCalculatorError :: ReprConflict => { let guar = cx . tcx () . dcx () . delayed_bug (format ! ("computed impossible repr (packed enum?): {ty:?}")) ; LayoutError :: ReferencesError (guar) } LayoutCalculatorError :: ZeroLengthSimdType => { cx . tcx () . dcx () . emit_fatal (ZeroLengthSimdType { ty }) } LayoutCalculatorError :: OversizedSimdType { max_lanes } => { cx . tcx () . dcx () . emit_fatal (OversizedSimdType { ty , max_lanes }) } LayoutCalculatorError :: NonPrimitiveSimdType (field) => { cx . tcx () . dcx () . emit_fatal (NonPrimitiveSimdType { ty , e_ty : field . ty }) } } ; error (cx , err) }
    };
}

map_error!();