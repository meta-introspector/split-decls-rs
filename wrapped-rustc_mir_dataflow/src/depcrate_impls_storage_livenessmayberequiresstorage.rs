// Generated macro for MaybeRequiresStorage (struct)
macro_rules! Depcrate_impls_storage_livenessMaybeRequiresStorage {
() => {
// Module: crate::impls::storage_liveness
// Provides: {"MaybeRequiresStorage"}
// Dependencies: {}
# [doc = " Dataflow analysis that determines whether each local requires storage at a"] # [doc = " given location; i.e. whether its storage can go away without being observed."] pub struct MaybeRequiresStorage < 'mir , 'tcx > { borrowed_locals : BorrowedLocalsResults < 'mir , 'tcx > , }
};
}
