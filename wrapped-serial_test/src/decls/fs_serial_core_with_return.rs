macro_rules! fs_serial_core_with_return {
    () => {
        # [doc (hidden)] pub fn fs_serial_core_with_return < E > (names : Vec < & str > , path : Option < & str > , function : fn () -> Result < () , E > ,) -> Result < () , E > { let mut locks = get_locks (& names , path) ; locks . iter_mut () . for_each (| lock | lock . start_serial ()) ; let res = panic :: catch_unwind (function) ; locks . into_iter () . for_each (| lock | lock . end_serial ()) ; match res { Ok (ret) => ret , Err (err) => { panic :: resume_unwind (err) ; } } }
    };
}

fs_serial_core_with_return!()