macro_rules! fs_parallel_core_with_return {
    () => {
        # [doc (hidden)] pub fn fs_parallel_core_with_return < E > (names : Vec < & str > , path : Option < & str > , function : fn () -> Result < () , E > ,) -> Result < () , E > { get_locks (& names , path) . iter_mut () . for_each (| lock | lock . start_parallel ()) ; let res = panic :: catch_unwind (function) ; get_locks (& names , path) . into_iter () . for_each (| lock | lock . end_parallel ()) ; match res { Ok (ret) => ret , Err (err) => { panic :: resume_unwind (err) ; } } }
    };
}

fs_parallel_core_with_return!();