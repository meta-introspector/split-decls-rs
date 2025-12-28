macro_rules! deps {
    () => {
        TraitRef!();
        PredicatePolarity!();
    };
}

macro_rules! TraitPredicate {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct TraitPredicate { pub trait_ref : TraitRef , pub polarity : PredicatePolarity , }
    };
}

TraitPredicate!()