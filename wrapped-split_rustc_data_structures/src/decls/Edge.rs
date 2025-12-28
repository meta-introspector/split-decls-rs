macro_rules! deps {
    () => {
        Index!();
    };
}

macro_rules! Edge {
    () => {
        deps!();
        # [derive (Clone , PartialEq , Eq , Debug , Hash)] struct Edge { source : Index , target : Index , }
    };
}

Edge!();