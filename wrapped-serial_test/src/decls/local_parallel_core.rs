macro_rules! local_parallel_core {
    () => {
        # [doc (hidden)] pub fn local_parallel_core (names : Vec < & str > , _path : Option < & str > , function : fn ()) { let locks = get_locks (names) ; locks . iter () . for_each (| lock | lock . start_parallel ()) ; let res = panic :: catch_unwind (| | { function () ; }) ; locks . iter () . for_each (| lock | lock . end_parallel ()) ; if let Err (err) = res { panic :: resume_unwind (err) ; } }
    };
}

local_parallel_core!();