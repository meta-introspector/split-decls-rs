macro_rules! deps {
    () => {
        GenericArgs!();
        Predicate!();
        Interner!();
        GoalSource!();
        Probe!();
        Certainty!();
        Goal!();
        CanonicalState!();
    };
}

macro_rules! ProbeStep {
    () => {
        deps!();
        # [derive_where (PartialEq , Eq , Hash , Debug ; I : Interner)] pub enum ProbeStep < I : Interner > { # [doc = " We added a goal to the `EvalCtxt` which will get proven"] # [doc = " the next time `EvalCtxt::try_evaluate_added_goals` is called."] AddGoal (GoalSource , CanonicalState < I , Goal < I , I :: Predicate > >) , # [doc = " A call to `probe` while proving the current goal. This is"] # [doc = " used whenever there are multiple candidates to prove the"] # [doc = " current goal."] NestedProbe (Probe < I >) , # [doc = " A trait goal was satisfied by an impl candidate."] RecordImplArgs { impl_args : CanonicalState < I , I :: GenericArgs > } , # [doc = " A call to `EvalCtxt::evaluate_added_goals_make_canonical_response` with"] # [doc = " `Certainty` was made. This is the certainty passed in, so it's not unified"] # [doc = " with the certainty of the `try_evaluate_added_goals` that is done within;"] # [doc = " if it's `Certainty::Yes`, then we can trust that the candidate is \"finished\""] # [doc = " and we didn't force ambiguity for some reason."] MakeCanonicalResponse { shallow_certainty : Certainty } , }
    };
}

ProbeStep!();