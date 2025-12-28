macro_rules! AtomicIterationCount {
    () => {
        # [derive (Debug)] pub (crate) struct AtomicIterationCount (AtomicU8) ;
    };
}

AtomicIterationCount!();