macro_rules! has_significant_drop_raw {
    () => {
        fn has_significant_drop_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > > ,) -> bool { let res = drop_tys_helper (tcx , query . value , query . typing_env , adt_consider_insignificant_dtor (tcx) , true , false ,) . filter (filter_array_elements (tcx , query . typing_env)) . next () . is_some () ; debug ! ("has_significant_drop_raw({:?}) = {:?}" , query , res) ; res }
    };
}

has_significant_drop_raw!();