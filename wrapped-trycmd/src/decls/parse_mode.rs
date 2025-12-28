macro_rules! deps {
    () => {
        Mode!();
    };
}

macro_rules! parse_mode {
    () => {
        deps!();
        fn parse_mode (var : Option < & std :: ffi :: OsStr >) -> crate :: Mode { if var == Some (std :: ffi :: OsStr :: new ("overwrite")) { crate :: Mode :: Overwrite } else if var == Some (std :: ffi :: OsStr :: new ("dump")) { crate :: Mode :: Dump ("dump" . into ()) } else { crate :: Mode :: Fail } }
    };
}

parse_mode!()