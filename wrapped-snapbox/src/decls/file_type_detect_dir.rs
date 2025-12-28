macro_rules! deps {
    () => {
        FileType!();
    };
}

macro_rules! file_type_detect_dir {
    () => {
        deps!();
        # [test] fn file_type_detect_dir () { let path = std :: path :: Path :: new (env ! ("CARGO_MANIFEST_DIR")) ; dbg ! (path) ; let actual = FileType :: from_path (path) ; assert_eq ! (actual , FileType :: Dir) ; }
    };
}

file_type_detect_dir!()