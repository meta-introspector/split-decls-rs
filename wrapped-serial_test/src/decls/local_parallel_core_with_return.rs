macro_rules! local_parallel_core_with_return {
    () => {
        # [doc (hidden)] pub fn local_parallel_core_with_return < E > (names : Vec < & str > , _path : Option < & str > , function : fn () -> Result < () , E > ,) -> Result < () , E > { let locks = get_locks (names) ; locks . iter () . for_each (| lock | lock . start_parallel ()) ; let res = panic :: catch_unwind (function) ; locks . iter () . for_each (| lock | lock . end_parallel ()) ; match res { Ok (ret) => ret , Err (err) => { panic :: resume_unwind (err) ; } } }
    };
}

local_parallel_core_with_return!();