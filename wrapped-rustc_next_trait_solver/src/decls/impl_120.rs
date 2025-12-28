macro_rules! deps {
    () => {
        SolverDelegate!();
        EvalCtxt!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < D , I > EvalCtxt < '_ , D > where D : SolverDelegate < Interner = I > , I : Interner , { # [instrument (level = "trace" , skip (self) , ret)] pub (super) fn compute_projection_goal (& mut self , goal : Goal < I , ProjectionPredicate < I > > ,) -> QueryResult < I > { let cx = self . cx () ; let projection_term = goal . predicate . projection_term . to_term (cx) ; let goal = goal . with (cx , ty :: PredicateKind :: AliasRelate (projection_term , goal . predicate . term , ty :: AliasRelationDirection :: Equate ,) ,) ; self . add_goal (GoalSource :: TypeRelating , goal) ; self . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) } }
    };
}

impl_120!()