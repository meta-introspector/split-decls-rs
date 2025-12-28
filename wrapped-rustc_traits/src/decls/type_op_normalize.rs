macro_rules! type_op_normalize {
    () => {
        fn type_op_normalize < 'tcx , T > (ocx : & ObligationCtxt < '_ , 'tcx > , key : ParamEnvAnd < 'tcx , Normalize < T > > ,) -> Result < T , NoSolution > where T : fmt :: Debug + TypeFoldable < TyCtxt < 'tcx > > , { let ParamEnvAnd { param_env , value : Normalize { value } } = key ; let Normalized { value , obligations } = ocx . infcx . at (& ObligationCause :: dummy () , param_env) . query_normalize (value) ? ; ocx . register_obligations (obligations) ; Ok (value) }
    };
}

type_op_normalize!();