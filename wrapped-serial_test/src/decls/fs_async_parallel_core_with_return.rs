macro_rules! fs_async_parallel_core_with_return {
    () => {
        # [doc (hidden)] # [cfg (feature = "async")] pub async fn fs_async_parallel_core_with_return < E > (names : Vec < & str > , path : Option < & str > , fut : impl std :: future :: Future < Output = Result < () , E > > + panic :: UnwindSafe ,) -> Result < () , E > { get_locks (& names , path) . iter_mut () . for_each (| lock | lock . start_parallel ()) ; let res = fut . catch_unwind () . await ; get_locks (& names , path) . into_iter () . for_each (| lock | lock . end_parallel ()) ; match res { Ok (ret) => ret , Err (err) => { panic :: resume_unwind (err) ; } } }
    };
}

fs_async_parallel_core_with_return!()