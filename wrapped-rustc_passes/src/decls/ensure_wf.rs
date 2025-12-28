macro_rules! ensure_wf {
    () => {
        pub fn ensure_wf < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , ty : Ty < 'tcx > , def_id : LocalDefId , span : Span ,) -> bool { let (infcx , param_env) = tcx . infer_ctxt () . build_with_typing_env (typing_env) ; let ocx = traits :: ObligationCtxt :: new_with_diagnostics (& infcx) ; let pred = ty :: ClauseKind :: WellFormed (ty . into ()) ; let obligation = traits :: Obligation :: new (tcx , traits :: ObligationCause :: new (span , def_id , traits :: ObligationCauseCode :: WellFormed (Some (traits :: WellFormedLoc :: Ty (def_id))) ,) , param_env , pred ,) ; ocx . register_obligation (obligation) ; let errors = ocx . select_all_or_error () ; if ! errors . is_empty () { infcx . err_ctxt () . report_fulfillment_errors (errors) ; false } else { true } }
    };
}

ensure_wf!()