macro_rules! MockAttributeReaderTyCtxt {
    () => {
        pub struct MockAttributeReaderTyCtxt < 'tcx > (pub TyCtxt < 'tcx >) ;
    };
}

MockAttributeReaderTyCtxt!();