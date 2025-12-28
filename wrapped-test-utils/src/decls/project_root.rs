macro_rules! project_root {
    () => {
        # [doc = " Returns the path to the root directory of `rust-analyzer` project."] pub fn project_root () -> Utf8PathBuf { let dir = env ! ("CARGO_MANIFEST_DIR") ; Utf8PathBuf :: from_path_buf (PathBuf :: from (dir) . parent () . unwrap () . parent () . unwrap () . to_owned ()) . unwrap () }
    };
}

project_root!();