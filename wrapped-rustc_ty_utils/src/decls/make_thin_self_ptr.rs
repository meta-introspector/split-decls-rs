macro_rules! make_thin_self_ptr {
    () => {
        # [tracing :: instrument (level = "debug" , skip (cx))] fn make_thin_self_ptr < 'tcx > (cx : & (impl HasTyCtxt < 'tcx > + HasTypingEnv < 'tcx >) , layout : TyAndLayout < 'tcx > ,) -> TyAndLayout < 'tcx > { let tcx = cx . tcx () ; let wide_pointer_ty = if layout . is_unsized () { Ty :: new_mut_ptr (tcx , layout . ty) } else { match layout . backend_repr { BackendRepr :: ScalarPair (..) | BackendRepr :: Scalar (..) => () , _ => bug ! ("receiver type has unsupported layout: {:?}" , layout) , } let mut wide_pointer_layout = layout ; while ! wide_pointer_layout . ty . is_raw_ptr () && ! wide_pointer_layout . ty . is_ref () { wide_pointer_layout = wide_pointer_layout . non_1zst_field (cx) . expect ("not exactly one non-1-ZST field in a `DispatchFromDyn` type") . 1 } wide_pointer_layout . ty } ; let unit_ptr_ty = Ty :: new_mut_ptr (tcx , tcx . types . unit) ; TyAndLayout { ty : wide_pointer_ty , .. tcx . layout_of (ty :: TypingEnv :: fully_monomorphized () . as_query_input (unit_ptr_ty)) . unwrap () } }
    };
}

make_thin_self_ptr!();