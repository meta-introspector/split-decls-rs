macro_rules! type_op_prove_predicate_with_cause {
    () => {
        # [doc = " The core of the `type_op_prove_predicate` query: for diagnostics purposes in NLL HRTB errors,"] # [doc = " this query can be re-run to better track the span of the obligation cause, and improve the error"] # [doc = " message. Do not call directly unless you're in that very specific context."] pub fn type_op_prove_predicate_with_cause < 'tcx > (ocx : & ObligationCtxt < '_ , 'tcx > , key : ParamEnvAnd < 'tcx , ProvePredicate < 'tcx > > , cause : ObligationCause < 'tcx > ,) { let ParamEnvAnd { param_env , value : ProvePredicate { predicate } } = key ; ocx . register_obligation (Obligation :: new (ocx . infcx . tcx , cause , param_env , predicate)) ; }
    };
}

type_op_prove_predicate_with_cause!();