macro_rules! local_async_parallel_core {
    () => {
        # [doc (hidden)] # [cfg (feature = "async")] pub async fn local_async_parallel_core (names : Vec < & str > , _path : Option < & str > , fut : impl std :: future :: Future < Output = () > + panic :: UnwindSafe ,) { let locks = get_locks (names) ; locks . iter () . for_each (| lock | lock . start_parallel ()) ; let res = fut . catch_unwind () . await ; locks . iter () . for_each (| lock | lock . end_parallel ()) ; if let Err (err) = res { panic :: resume_unwind (err) ; } }
    };
}

local_async_parallel_core!();