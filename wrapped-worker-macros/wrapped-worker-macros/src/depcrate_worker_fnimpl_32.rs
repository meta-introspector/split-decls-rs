// Generated macro for impl_32 (impl)
macro_rules! Depcrate_worker_fnimpl_32 {
() => {
// Module: crate::worker_fn
// Provides: {"impl_32"}
// Dependencies: {}
impl Parse for WorkerName { fn parse (input : ParseStream) -> syn :: Result < Self > { if input . is_empty () { return Ok (Self { worker_name : None }) ; } let worker_name = input . parse () ? ; Ok (Self { worker_name : Some (worker_name) , }) } }
};
}
