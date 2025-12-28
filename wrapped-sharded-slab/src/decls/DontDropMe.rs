macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! DontDropMe {
    () => {
        deps!();
        # [derive (Default , Debug)] struct DontDropMe (Arc < State >) ;
    };
}

DontDropMe!();