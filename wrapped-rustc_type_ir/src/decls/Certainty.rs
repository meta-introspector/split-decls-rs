macro_rules! deps {
    () => {
        MaybeCause!();
    };
}

macro_rules! Certainty {
    () => {
        deps!();
        # [derive (Clone , Copy , Hash , PartialEq , Eq , Debug)] # [cfg_attr (feature = "nightly" , derive (HashStable_NoContext))] pub enum Certainty { Yes , Maybe (MaybeCause) , }
    };
}

Certainty!();