macro_rules! deps {
    () => {
        Inner!();
        Node!();
    };
}

macro_rules! Kind {
    () => {
        deps!();
        # [derive (Clone , Debug)] enum Kind < Node : Idx > { # [doc = " A representation optimized for a small path graphs."] Path , General (Inner < Node >) , }
    };
}

Kind!()