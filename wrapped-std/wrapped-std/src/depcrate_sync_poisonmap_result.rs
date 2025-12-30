// Generated macro for map_result (function)
macro_rules! Depcrate_sync_poisonmap_result {
() => {
// Module: crate::sync::poison
// Provides: {"map_result"}
// Dependencies: {}
pub (crate) fn map_result < T , U , F > (result : LockResult < T > , f : F) -> LockResult < U > where F : FnOnce (T) -> U , { match result { Ok (t) => Ok (f (t)) , # [cfg (panic = "unwind")] Err (PoisonError { data }) => Err (PoisonError :: new (f (data))) , } }
};
}
