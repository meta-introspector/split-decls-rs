macro_rules! fs_parallel_core {
    () => {
        # [doc (hidden)] pub fn fs_parallel_core (names : Vec < & str > , path : Option < & str > , function : fn ()) { get_locks (& names , path) . iter_mut () . for_each (| lock | lock . start_parallel ()) ; let res = panic :: catch_unwind (| | { function () ; }) ; get_locks (& names , path) . into_iter () . for_each (| lock | lock . end_parallel ()) ; if let Err (err) = res { panic :: resume_unwind (err) ; } }
    };
}

fs_parallel_core!();