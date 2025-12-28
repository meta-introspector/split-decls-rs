macro_rules! deps {
    () => {
        DepNode!();
        EdgesVec!();
    };
}

macro_rules! NodeInfo {
    () => {
        deps!();
        # [derive (Debug)] struct NodeInfo { node : DepNode , fingerprint : Fingerprint , edges : EdgesVec , }
    };
}

NodeInfo!()