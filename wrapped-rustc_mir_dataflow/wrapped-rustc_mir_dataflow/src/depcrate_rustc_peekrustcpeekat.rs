// Generated macro for RustcPeekAt (trait)
macro_rules! Depcrate_rustc_peekRustcPeekAt {
() => {
// Module: crate::rustc_peek
// Provides: {"RustcPeekAt"}
// Dependencies: {}
trait RustcPeekAt < 'tcx > : Analysis < 'tcx > { fn peek_at (& self , tcx : TyCtxt < 'tcx > , place : mir :: Place < 'tcx > , state : & Self :: Domain , call : PeekCall ,) ; }
};
}
