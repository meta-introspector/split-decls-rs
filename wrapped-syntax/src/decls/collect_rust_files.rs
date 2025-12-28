macro_rules! collect_rust_files {
    () => {
        # [doc = " Collects all `.rs` files from `dir` subdirectories defined by `paths`."] fn collect_rust_files (root_dir : & Path , paths : & [& str]) -> Vec < (PathBuf , String) > { paths . iter () . flat_map (| path | { let path = root_dir . to_owned () . join (path) ; rust_files_in_dir (& path) . into_iter () }) . map (| path | { let text = read_text (& path) ; (path , text) }) . collect () }
    };
}

collect_rust_files!()