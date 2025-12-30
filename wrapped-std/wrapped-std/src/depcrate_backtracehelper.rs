// Generated macro for helper (module)
macro_rules! Depcrate_backtracehelper {
() => {
// Module: crate::backtrace
// Provides: {"helper"}
// Dependencies: {}
mod helper { use super :: * ; pub (super) type LazyResolve = impl (FnOnce () -> Capture) + Send + Sync + UnwindSafe ; # [define_opaque (LazyResolve)] pub (super) fn lazy_resolve (mut capture : Capture) -> LazyResolve { move | | { let _lock = lock () ; for frame in capture . frames . iter_mut () { let symbols = & mut frame . symbols ; let frame = match & frame . frame { RawFrame :: Actual (frame) => frame , # [cfg (test)] RawFrame :: Fake => unimplemented ! () , } ; unsafe { backtrace_rs :: resolve_frame_unsynchronized (frame , | symbol | { symbols . push (BacktraceSymbol { name : symbol . name () . map (| m | m . as_bytes () . to_vec ()) , filename : symbol . filename_raw () . map (| b | match b { BytesOrWideString :: Bytes (b) => BytesOrWide :: Bytes (b . to_owned ()) , BytesOrWideString :: Wide (b) => BytesOrWide :: Wide (b . to_owned ()) , }) , lineno : symbol . lineno () , colno : symbol . colno () , }) ; }) ; } } capture } } }
};
}
