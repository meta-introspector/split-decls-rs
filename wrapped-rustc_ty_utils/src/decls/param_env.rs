macro_rules! deps {
    () => {
        ImplTraitInTraitFinder!();
    };
}

macro_rules! param_env {
    () => {
        deps!();
        # [doc = " See `ParamEnv` struct definition for details."] fn param_env (tcx : TyCtxt < '_ > , def_id : DefId) -> ty :: ParamEnv < '_ > { let ty :: InstantiatedPredicates { mut predicates , .. } = tcx . predicates_of (def_id) . instantiate_identity (tcx) ; if tcx . def_kind (def_id) == DefKind :: AssocFn && let assoc_item = tcx . associated_item (def_id) && assoc_item . container == ty :: AssocContainer :: Trait && assoc_item . defaultness (tcx) . has_value () { let sig = tcx . fn_sig (def_id) . instantiate_identity () ; sig . skip_binder () . visit_with (& mut ImplTraitInTraitFinder { tcx , fn_def_id : def_id , bound_vars : sig . bound_vars () , predicates : & mut predicates , seen : FxHashSet :: default () , depth : ty :: INNERMOST , }) ; } if tcx . is_conditionally_const (def_id) { predicates . extend (tcx . const_conditions (def_id) . instantiate_identity (tcx) . into_iter () . map (| (trait_ref , _) | trait_ref . to_host_effect_clause (tcx , ty :: BoundConstness :: Maybe) ,) ,) ; } let local_did = def_id . as_local () ; let unnormalized_env = ty :: ParamEnv :: new (tcx . mk_clauses (& predicates)) ; let body_id = local_did . unwrap_or (CRATE_DEF_ID) ; let cause = traits :: ObligationCause :: misc (tcx . def_span (def_id) , body_id) ; traits :: normalize_param_env_or_error (tcx , unnormalized_env , cause) }
    };
}

param_env!()