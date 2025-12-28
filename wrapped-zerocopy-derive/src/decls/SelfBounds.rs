macro_rules! deps {
    () => {
        Trait!();
    };
}

macro_rules! SelfBounds {
    () => {
        deps!();
        # [derive (Debug , Eq , PartialEq)] enum SelfBounds < 'a > { None , All (& 'a [Trait]) , }
    };
}

SelfBounds!();