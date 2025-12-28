macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! StateData {
    () => {
        deps!();
        # [doc = " See [`State`]."] # [derive (PartialEq , Eq , Debug)] pub struct StateData < V > { bottom : V , # [doc = " This map only contains values that are not `⊥`."] map : FxHashMap < ValueIndex , V > , }
    };
}

StateData!();