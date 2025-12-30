// Generated macro for thread (module)
macro_rules! Depcrate_webthread {
() => {
// Module: crate::web
// Provides: {"thread"}
// Dependencies: {}
# [cfg (not (all (target_family = "wasm" , target_os = "unknown")))] mod thread { pub (super) struct ScopeFuture < 'scope , 'env , F , T > (& 'scope & 'env (F , T)) ; pub (super) struct YieldNowFuture ; }
};
}
