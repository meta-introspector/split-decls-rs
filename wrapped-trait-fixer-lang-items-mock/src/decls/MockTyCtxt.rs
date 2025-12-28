macro_rules! MockTyCtxt {
    () => {
        pub struct MockTyCtxt < 'tcx > (pub TyCtxt < 'tcx >) ;
    };
}

MockTyCtxt!();