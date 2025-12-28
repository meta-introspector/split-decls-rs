macro_rules! deps {
    () => {
        Trait!();
    };
}

macro_rules! TraitBound {
    () => {
        deps!();
        # [derive (Debug , Eq , PartialEq)] enum TraitBound { Slf , Other (Trait) , }
    };
}

TraitBound!()