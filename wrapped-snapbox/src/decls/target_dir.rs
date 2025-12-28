macro_rules! target_dir {
    () => {
        fn target_dir () -> std :: path :: PathBuf { std :: env :: current_exe () . ok () . map (| mut path | { path . pop () ; if path . ends_with ("deps") { path . pop () ; } path }) . unwrap () }
    };
}

target_dir!()