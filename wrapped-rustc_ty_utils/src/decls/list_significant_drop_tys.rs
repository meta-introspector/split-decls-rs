macro_rules! list_significant_drop_tys {
    () => {
        # [instrument (level = "debug" , skip (tcx) , ret)] fn list_significant_drop_tys < 'tcx > (tcx : TyCtxt < 'tcx > , key : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > > ,) -> & 'tcx ty :: List < Ty < 'tcx > > { tcx . mk_type_list (& drop_tys_helper (tcx , key . value , key . typing_env , adt_consider_insignificant_dtor (tcx) , true , true ,) . filter_map (| res | res . ok ()) . collect :: < Vec < _ > > () ,) }
    };
}

list_significant_drop_tys!();