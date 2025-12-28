macro_rules! fs_serial_core {
    () => {
        # [doc (hidden)] pub fn fs_serial_core (names : Vec < & str > , path : Option < & str > , function : fn ()) { assert ! (names . len () > 0) ; let mut locks = get_locks (& names , path) ; locks . iter_mut () . for_each (| lock | lock . start_serial ()) ; let res = panic :: catch_unwind (function) ; locks . into_iter () . for_each (| lock | lock . end_serial ()) ; if let Err (err) = res { panic :: resume_unwind (err) ; } }
    };
}

fs_serial_core!()