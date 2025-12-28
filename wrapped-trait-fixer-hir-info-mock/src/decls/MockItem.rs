macro_rules! MockItem {
    () => {
        pub struct MockItem < 'tcx > (pub Item < 'tcx > , PhantomData < & 'tcx () >) ;
    };
}

MockItem!()