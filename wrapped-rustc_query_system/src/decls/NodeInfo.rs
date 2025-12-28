macro_rules! deps {
    () => {
        EdgesVec!();
        DepNode!();
    };
}

macro_rules! NodeInfo {
    () => {
        deps!();
        # [derive (Debug)] struct NodeInfo { node : DepNode , fingerprint : Fingerprint , edges : EdgesVec , }
    };
}

NodeInfo!();