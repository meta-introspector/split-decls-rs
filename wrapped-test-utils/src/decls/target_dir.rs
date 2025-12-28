macro_rules! target_dir {
    () => {
        pub fn target_dir () -> Utf8PathBuf { match std :: env :: var ("CARGO_TARGET_DIR") { Ok (target) => Utf8PathBuf :: from (target) , Err (_) => project_root () . join ("target") , } }
    };
}

target_dir!()