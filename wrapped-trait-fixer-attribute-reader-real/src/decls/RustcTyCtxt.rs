macro_rules! RustcTyCtxt {
    () => {
        pub struct RustcTyCtxt < 'tcx > (pub TyCtxt < 'tcx >) ;
    };
}

RustcTyCtxt!()