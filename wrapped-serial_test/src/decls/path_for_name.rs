macro_rules! path_for_name {
    () => {
        pub (crate) fn path_for_name (name : & str) -> String { let mut pathbuf = env :: temp_dir () ; pathbuf . push (format ! ("serial-test-{}" , name)) ; pathbuf . into_os_string () . into_string () . unwrap () }
    };
}

path_for_name!()