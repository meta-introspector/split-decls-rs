macro_rules! deps {
    () => {
        SolverDelegate!();
        EvalCtxt!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < D , I > EvalCtxt < '_ , D > where D : SolverDelegate < Interner = I > , I : Interner , { # [instrument (level = "trace" , skip (self) , ret)] pub (super) fn normalize_anon_const (& mut self , goal : Goal < I , ty :: NormalizesTo < I > > ,) -> QueryResult < I > { if let Some (normalized_const) = self . evaluate_const (goal . param_env , ty :: UnevaluatedConst :: new (goal . predicate . alias . def_id , goal . predicate . alias . args) ,) { self . instantiate_normalizes_to_term (goal , normalized_const . into ()) ; self . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) } else { self . evaluate_added_goals_and_make_canonical_response (Certainty :: AMBIGUOUS) } } }
    };
}

impl_108!();