macro_rules! deps {
    () => {
        FileType!();
    };
}

macro_rules! file_type_detect_file {
    () => {
        deps!();
        # [test] fn file_type_detect_file () { let path = std :: path :: Path :: new (env ! ("CARGO_MANIFEST_DIR")) . join ("Cargo.toml") ; dbg ! (& path) ; let actual = FileType :: from_path (& path) ; assert_eq ! (actual , FileType :: File) ; }
    };
}

file_type_detect_file!()