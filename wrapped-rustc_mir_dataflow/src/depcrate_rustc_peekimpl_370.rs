// Generated macro for impl_370 (impl)
macro_rules! Depcrate_rustc_peekimpl_370 {
() => {
// Module: crate::rustc_peek
// Provides: {"impl_370"}
// Dependencies: {}
impl < 'tcx > RustcPeekAt < 'tcx > for MaybeLiveLocals { fn peek_at (& self , tcx : TyCtxt < 'tcx > , place : mir :: Place < 'tcx > , state : & Self :: Domain , call : PeekCall ,) { info ! (? place , "peek_at") ; let Some (local) = place . as_local () else { tcx . dcx () . emit_err (PeekArgumentNotALocal { span : call . span }) ; return ; } ; if ! state . contains (local) { tcx . dcx () . emit_err (PeekBitNotSet { span : call . span }) ; } } }
};
}
