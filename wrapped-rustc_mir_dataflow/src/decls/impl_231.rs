macro_rules! deps {
    () => {
        PeekCall!();
        MaybeLiveLocals!();
        PeekBitNotSet!();
        PeekArgumentNotALocal!();
        RustcPeekAt!();
    };
}

macro_rules! impl_231 {
    () => {
        deps!();
        impl < 'tcx > RustcPeekAt < 'tcx > for MaybeLiveLocals { fn peek_at (& self , tcx : TyCtxt < 'tcx > , place : mir :: Place < 'tcx > , state : & Self :: Domain , call : PeekCall ,) { info ! (? place , "peek_at") ; let Some (local) = place . as_local () else { tcx . dcx () . emit_err (PeekArgumentNotALocal { span : call . span }) ; return ; } ; if ! state . contains (local) { tcx . dcx () . emit_err (PeekBitNotSet { span : call . span }) ; } } }
    };
}

impl_231!()