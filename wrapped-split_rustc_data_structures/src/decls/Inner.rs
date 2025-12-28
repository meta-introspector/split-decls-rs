macro_rules! deps {
    () => {
        Time!();
    };
}

macro_rules! Inner {
    () => {
        deps!();
        # [doc = " Tracks the list of dominators for each node."] # [derive (Clone , Debug)] struct Inner < N : Idx > { immediate_dominators : IndexVec < N , Option < N > > , time : IndexVec < N , Time > , }
    };
}

Inner!()