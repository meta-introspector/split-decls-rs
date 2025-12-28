macro_rules! deps {
    () => {
        FileType!();
    };
}

macro_rules! file_type_detect_missing {
    () => {
        deps!();
        # [test] fn file_type_detect_missing () { let path = std :: path :: Path :: new ("this-should-never-exist") ; dbg ! (path) ; let actual = FileType :: from_path (path) ; assert_eq ! (actual , FileType :: Missing) ; }
    };
}

file_type_detect_missing!();