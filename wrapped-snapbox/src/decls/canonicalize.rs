macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! canonicalize {
    () => {
        deps!();
        pub (crate) fn canonicalize (path : & std :: path :: Path) -> Result < std :: path :: PathBuf , std :: io :: Error > { # [cfg (feature = "dir")] { dunce :: canonicalize (path) } # [cfg (not (feature = "dir"))] { Ok (strip_trailing_slash (path) . to_owned ()) } }
    };
}

canonicalize!();