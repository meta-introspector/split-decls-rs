macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! resolve_dir {
    () => {
        deps!();
        pub fn resolve_dir (path : impl AsRef < std :: path :: Path > ,) -> Result < std :: path :: PathBuf , std :: io :: Error > { let path = path . as_ref () ; let meta = std :: fs :: symlink_metadata (path) ? ; if meta . is_dir () { canonicalize (path) } else if meta . is_file () { let target = std :: fs :: read_to_string (path) ? ; let target_path = path . parent () . unwrap () . join (target) ; resolve_dir (target_path) } else { canonicalize (path) } }
    };
}

resolve_dir!()