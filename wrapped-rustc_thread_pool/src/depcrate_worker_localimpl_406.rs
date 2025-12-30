// Generated macro for impl_406 (impl)
macro_rules! Depcrate_worker_localimpl_406 {
() => {
// Module: crate::worker_local
// Provides: {"impl_406"}
// Dependencies: {}
impl < T > WorkerLocal < Vec < T > > { # [doc = " Joins the elements of all the worker locals into one Vec"] pub fn join (self) -> Vec < T > { self . into_inner () . into_iter () . flat_map (| v | v) . collect () } }
};
}
