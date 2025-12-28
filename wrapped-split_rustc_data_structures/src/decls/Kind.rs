macro_rules! deps {
    () => {
        Node!();
        Inner!();
    };
}

macro_rules! Kind {
    () => {
        deps!();
        # [derive (Clone , Debug)] enum Kind < Node : Idx > { # [doc = " A representation optimized for a small path graphs."] Path , General (Inner < Node >) , }
    };
}

Kind!();