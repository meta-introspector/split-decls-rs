macro_rules! deps {
    () => {
        CycleError!();
        QueryJobId!();
    };
}

macro_rules! QueryWaiter {
    () => {
        deps!();
        # [derive (Debug)] struct QueryWaiter < I > { query : Option < QueryJobId > , condvar : Condvar , span : Span , cycle : Mutex < Option < CycleError < I > > > , }
    };
}

QueryWaiter!()