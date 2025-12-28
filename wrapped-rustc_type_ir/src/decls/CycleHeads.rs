macro_rules! deps {
    () => {
        CycleHead!();
    };
}

macro_rules! CycleHeads {
    () => {
        deps!();
        # [doc = " All cycle heads a given goal depends on, ordered by their stack depth."] # [doc = ""] # [doc = " We also track all paths from this goal to that head. This is necessary"] # [doc = " when rebasing provisional cache results."] # [derive (Clone , Debug , Default)] struct CycleHeads { heads : BTreeMap < StackDepth , CycleHead > , }
    };
}

CycleHeads!()