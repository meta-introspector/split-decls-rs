macro_rules! MockTyCtxtWrapper {
    () => {
        pub struct MockTyCtxtWrapper < 'tcx > (pub TyCtxt < 'tcx >) ;
    };
}

MockTyCtxtWrapper!()