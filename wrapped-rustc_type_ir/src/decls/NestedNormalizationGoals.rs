macro_rules! deps {
    () => {
        Predicate!();
        Interner!();
        GoalSource!();
        Goal!();
    };
}

macro_rules! NestedNormalizationGoals {
    () => {
        deps!();
        # [derive_where (Clone , Hash , PartialEq , Debug , Default ; I : Interner)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic)] # [cfg_attr (feature = "nightly" , derive (HashStable_NoContext))] pub struct NestedNormalizationGoals < I : Interner > (pub Vec < (GoalSource , Goal < I , I :: Predicate >) >) ;
    };
}

NestedNormalizationGoals!();