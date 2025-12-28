macro_rules! cargo_rustc_current_dir {
    () => {
        # [doc = " Find the base directory for [`std::file!`]"] # [doc (hidden)] # [macro_export] macro_rules ! cargo_rustc_current_dir { () => { { if let Some (rustc_root) = :: std :: option_env ! ("CARGO_RUSTC_CURRENT_DIR") { :: std :: path :: Path :: new (rustc_root) } else { let manifest_dir = :: std :: path :: Path :: new (:: std :: env ! ("CARGO_MANIFEST_DIR")) ; manifest_dir . ancestors () . filter (| it | it . join ("Cargo.toml") . exists ()) . last () . unwrap () } } } ; }
    };
}

cargo_rustc_current_dir!()