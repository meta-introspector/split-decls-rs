macro_rules! rust_files_in_dir {
    () => {
        # [doc = " Collects paths to all `.rs` files from `dir` in a sorted `Vec<PathBuf>`."] fn rust_files_in_dir (dir : & Path) -> Vec < PathBuf > { let mut acc = Vec :: new () ; for file in fs :: read_dir (dir) . unwrap () { let file = file . unwrap () ; let path = file . path () ; if path . extension () . unwrap_or_default () == "rs" { acc . push (path) ; } } acc . sort () ; acc }
    };
}

rust_files_in_dir!();