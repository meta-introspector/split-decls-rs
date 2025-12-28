macro_rules! deps {
    () => {
        WipProbe!();
    };
}

macro_rules! WipProbeStep {
    () => {
        deps!();
        # [derive_where (PartialEq , Debug ; I : Interner)] enum WipProbeStep < I : Interner > { AddGoal (GoalSource , inspect :: CanonicalState < I , Goal < I , I :: Predicate > >) , NestedProbe (WipProbe < I >) , MakeCanonicalResponse { shallow_certainty : Certainty } , RecordImplArgs { impl_args : inspect :: CanonicalState < I , I :: GenericArgs > } , }
    };
}

WipProbeStep!()