macro_rules! deps {
    () => {
        PeekCall!();
        Analysis!();
    };
}

macro_rules! RustcPeekAt {
    () => {
        deps!();
        trait RustcPeekAt < 'tcx > : Analysis < 'tcx > { fn peek_at (& self , tcx : TyCtxt < 'tcx > , place : mir :: Place < 'tcx > , state : & Self :: Domain , call : PeekCall ,) ; }
    };
}

RustcPeekAt!();