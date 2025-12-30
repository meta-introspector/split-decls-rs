// Generated macro for local_async_parallel_core_with_return (function)
macro_rules! Depcrate_parallel_code_locklocal_async_parallel_core_with_return {
() => {
// Module: crate::parallel_code_lock
// Provides: {"local_async_parallel_core_with_return"}
// Dependencies: {}
# [doc (hidden)] # [cfg (feature = "async")] pub async fn local_async_parallel_core_with_return < E > (names : Vec < & str > , _path : Option < & str > , fut : impl std :: future :: Future < Output = Result < () , E > > + panic :: UnwindSafe ,) -> Result < () , E > { let locks = get_locks (names) ; locks . iter () . for_each (| lock | lock . start_parallel ()) ; let res = fut . catch_unwind () . await ; locks . iter () . for_each (| lock | lock . end_parallel ()) ; match res { Ok (ret) => ret , Err (err) => { panic :: resume_unwind (err) ; } } }
};
}
