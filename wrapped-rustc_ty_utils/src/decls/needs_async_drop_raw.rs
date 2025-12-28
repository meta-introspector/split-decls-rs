macro_rules! deps {
    () => {
        DtorType!();
    };
}

macro_rules! needs_async_drop_raw {
    () => {
        deps!();
        fn needs_async_drop_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > > ,) -> bool { let adt_has_async_dtor = | adt_def : ty :: AdtDef < 'tcx > | adt_def . async_destructor (tcx) . map (| _ | DtorType :: Significant) ; let res = drop_tys_helper (tcx , query . value , query . typing_env , adt_has_async_dtor , false , false) . filter (filter_array_elements_async (tcx , query . typing_env)) . next () . is_some () ; debug ! ("needs_async_drop_raw({:?}) = {:?}" , query , res) ; res }
    };
}

needs_async_drop_raw!();