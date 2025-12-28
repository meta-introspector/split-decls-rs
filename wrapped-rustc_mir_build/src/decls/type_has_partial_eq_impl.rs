macro_rules! deps {
    () => {
        PartialEqImplStatus!();
    };
}

macro_rules! type_has_partial_eq_impl {
    () => {
        deps!();
        # [instrument (level = "trace" , skip (tcx) , ret)] fn type_has_partial_eq_impl < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , ty : Ty < 'tcx > ,) -> PartialEqImplStatus { let (infcx , param_env) = tcx . infer_ctxt () . build_with_typing_env (typing_env) ; let partial_eq_trait_id = tcx . require_lang_item (hir :: LangItem :: PartialEq , DUMMY_SP) ; let structural_partial_eq_trait_id = tcx . require_lang_item (hir :: LangItem :: StructuralPeq , DUMMY_SP) ; let partial_eq_obligation = Obligation :: new (tcx , ObligationCause :: dummy () , param_env , ty :: TraitRef :: new (tcx , partial_eq_trait_id , [ty , ty]) ,) ; let mut automatically_derived = false ; let mut structural_peq = false ; let mut impl_def_id = None ; for def_id in tcx . non_blanket_impls_for_ty (partial_eq_trait_id , ty) { automatically_derived = find_attr ! (tcx . get_all_attrs (def_id) , AttributeKind :: AutomaticallyDerived (..)) ; impl_def_id = Some (def_id) ; } for _ in tcx . non_blanket_impls_for_ty (structural_partial_eq_trait_id , ty) { structural_peq = true ; } PartialEqImplStatus { has_impl : infcx . predicate_must_hold_modulo_regions (& partial_eq_obligation) , is_derived : automatically_derived , structural_partial_eq : structural_peq , non_blanket_impl : impl_def_id , } }
    };
}

type_has_partial_eq_impl!()