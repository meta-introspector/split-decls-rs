macro_rules! deps {
    () => {
        QueryStackFrameExtra!();
    };
}

macro_rules! QueryStackDeferred {
    () => {
        deps!();
        # [doc = " Track a 'side effect' for a particular query."] # [doc = " This is used to hold a closure which can create `QueryStackFrameExtra`."] # [derive (Clone)] pub struct QueryStackDeferred < 'tcx > { _dummy : PhantomData < & 'tcx () > , extract : Arc < dyn Fn () -> QueryStackFrameExtra + DynSync + DynSend > , }
    };
}

QueryStackDeferred!();