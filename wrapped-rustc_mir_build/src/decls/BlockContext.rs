macro_rules! deps {
    () => {
        BlockFrame!();
    };
}

macro_rules! BlockContext {
    () => {
        deps!();
        # [derive (Debug)] struct BlockContext (Vec < BlockFrame >) ;
    };
}

BlockContext!();