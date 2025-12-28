macro_rules! MockEarlyBinder {
    () => {
        pub struct MockEarlyBinder < 'tcx > (PhantomData < & 'tcx () >) ;
    };
}

MockEarlyBinder!();