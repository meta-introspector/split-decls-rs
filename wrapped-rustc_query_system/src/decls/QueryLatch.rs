macro_rules! deps {
    () => {
        QueryLatchInfo!();
    };
}

macro_rules! QueryLatch {
    () => {
        deps!();
        # [derive (Debug)] pub (super) struct QueryLatch < I > { info : Arc < Mutex < QueryLatchInfo < I > > > , }
    };
}

QueryLatch!();