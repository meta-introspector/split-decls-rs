macro_rules! deps {
    () => {
        EvalCtxt!();
        SolverDelegate!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < D , I > EvalCtxt < '_ , D > where D : SolverDelegate < Interner = I > , I : Interner , { pub (super) fn normalize_free_alias (& mut self , goal : Goal < I , ty :: NormalizesTo < I > > ,) -> QueryResult < I > { let cx = self . cx () ; let free_alias = goal . predicate . alias ; self . add_goals (GoalSource :: Misc , cx . predicates_of (free_alias . def_id) . iter_instantiated (cx , free_alias . args) . map (| pred | goal . with (cx , pred)) ,) ; let actual = if free_alias . kind (cx) . is_type () { cx . type_of (free_alias . def_id) . instantiate (cx , free_alias . args) } else { panic ! ("normalizing free const aliases in the type system is unsupported") ; } ; self . instantiate_normalizes_to_term (goal , actual . into ()) ; self . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) } }
    };
}

impl_110!()