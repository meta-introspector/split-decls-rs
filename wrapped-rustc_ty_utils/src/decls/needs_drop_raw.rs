macro_rules! deps {
    () => {
        DtorType!();
    };
}

macro_rules! needs_drop_raw {
    () => {
        deps!();
        fn needs_drop_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > > ,) -> bool { let adt_has_dtor = | adt_def : ty :: AdtDef < 'tcx > | adt_def . destructor (tcx) . map (| _ | DtorType :: Significant) ; let res = drop_tys_helper (tcx , query . value , query . typing_env , adt_has_dtor , false , false) . filter (filter_array_elements (tcx , query . typing_env)) . next () . is_some () ; debug ! ("needs_drop_raw({:?}) = {:?}" , query , res) ; res }
    };
}

needs_drop_raw!()