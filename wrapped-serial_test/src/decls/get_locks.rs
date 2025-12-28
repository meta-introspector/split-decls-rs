macro_rules! deps {
    () => {
        Lock!();
    };
}

macro_rules! get_locks {
    () => {
        deps!();
        pub (crate) fn get_locks (names : & Vec < & str > , path : Option < & str >) -> Vec < Lock > { if names . len () > 1 && path . is_some () { panic ! ("Can't do file_serial/parallel with both more than one name _and_ a specific path") ; } names . iter () . map (| name | make_lock_for_name_and_path (name , path)) . collect :: < Vec < _ > > () }
    };
}

get_locks!();