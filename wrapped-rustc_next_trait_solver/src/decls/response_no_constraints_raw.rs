macro_rules! response_no_constraints_raw {
    () => {
        fn response_no_constraints_raw < I : Interner > (cx : I , max_universe : ty :: UniverseIndex , variables : I :: CanonicalVarKinds , certainty : Certainty ,) -> CanonicalResponse < I > { ty :: Canonical { max_universe , variables , value : Response { var_values : ty :: CanonicalVarValues :: make_identity (cx , variables) , external_constraints : cx . mk_external_constraints (ExternalConstraintsData :: default ()) , certainty , } , } }
    };
}

response_no_constraints_raw!()