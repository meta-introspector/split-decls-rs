macro_rules! adt_significant_drop_tys {
    () => {
        fn adt_significant_drop_tys (tcx : TyCtxt < '_ > , def_id : DefId ,) -> Result < & ty :: List < Ty < '_ > > , AlwaysRequiresDrop > { drop_tys_helper (tcx , tcx . type_of (def_id) . instantiate_identity () , ty :: TypingEnv :: non_body_analysis (tcx , def_id) , adt_consider_insignificant_dtor (tcx) , true , false ,) . collect :: < Result < Vec < _ > , _ > > () . map (| components | tcx . mk_type_list (& components)) }
    };
}

adt_significant_drop_tys!();