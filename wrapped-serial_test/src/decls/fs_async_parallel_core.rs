macro_rules! fs_async_parallel_core {
    () => {
        # [doc (hidden)] # [cfg (feature = "async")] pub async fn fs_async_parallel_core (names : Vec < & str > , path : Option < & str > , fut : impl std :: future :: Future < Output = () > + panic :: UnwindSafe ,) { get_locks (& names , path) . iter_mut () . for_each (| lock | lock . start_parallel ()) ; let res = fut . catch_unwind () . await ; get_locks (& names , path) . into_iter () . for_each (| lock | lock . end_parallel ()) ; if let Err (err) = res { panic :: resume_unwind (err) ; } }
    };
}

fs_async_parallel_core!()