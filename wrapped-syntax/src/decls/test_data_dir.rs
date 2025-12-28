macro_rules! test_data_dir {
    () => {
        fn test_data_dir () -> PathBuf { project_root () . into_std_path_buf () . join ("crates/syntax/test_data") }
    };
}

test_data_dir!()