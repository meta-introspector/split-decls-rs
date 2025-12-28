macro_rules! deps {
    () => {
        EvalCtxt!();
        SolverDelegate!();
    };
}

macro_rules! instantiate_canonical_state {
    () => {
        deps!();
        pub fn instantiate_canonical_state < D , I , T : TypeFoldable < I > > (delegate : & D , span : I :: Span , param_env : I :: ParamEnv , orig_values : & mut Vec < I :: GenericArg > , state : inspect :: CanonicalState < I , T > ,) -> T where D : SolverDelegate < Interner = I > , I : Interner , { orig_values . extend (state . value . var_values . var_values . as_slice () [orig_values . len () ..] . iter () . map (| & arg | delegate . fresh_var_for_kind_with_span (arg , span)) ,) ; let instantiation = EvalCtxt :: compute_query_response_instantiation_values (delegate , orig_values , & state , span) ; let inspect :: State { var_values , data } = delegate . instantiate_canonical (state , instantiation) ; EvalCtxt :: unify_query_var_values (delegate , param_env , orig_values , var_values , span) ; data }
    };
}

instantiate_canonical_state!()