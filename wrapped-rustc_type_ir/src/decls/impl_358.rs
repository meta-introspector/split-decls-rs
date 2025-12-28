macro_rules! deps {
    () => {
        PredicatePolarity!();
    };
}

macro_rules! impl_358 {
    () => {
        deps!();
        impl PredicatePolarity { # [doc = " Flips polarity by turning `Positive` into `Negative` and `Negative` into `Positive`."] pub fn flip (& self) -> PredicatePolarity { match self { PredicatePolarity :: Positive => PredicatePolarity :: Negative , PredicatePolarity :: Negative => PredicatePolarity :: Positive , } } }
    };
}

impl_358!()