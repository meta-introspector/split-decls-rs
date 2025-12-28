macro_rules! is_item_raw {
    () => {
        fn is_item_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > > , item : LangItem ,) -> bool { let (infcx , param_env) = tcx . infer_ctxt () . build_with_typing_env (query . typing_env) ; let trait_def_id = tcx . require_lang_item (item , DUMMY_SP) ; traits :: type_known_to_meet_bound_modulo_regions (& infcx , param_env , query . value , trait_def_id) }
    };
}

is_item_raw!()