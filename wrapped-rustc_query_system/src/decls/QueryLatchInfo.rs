macro_rules! deps {
    () => {
        QueryWaiter!();
    };
}

macro_rules! QueryLatchInfo {
    () => {
        deps!();
        # [derive (Debug)] struct QueryLatchInfo < I > { complete : bool , waiters : Vec < Arc < QueryWaiter < I > > > , }
    };
}

QueryLatchInfo!();