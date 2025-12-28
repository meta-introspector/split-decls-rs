macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! populate_meta_dir {
    () => {
        deps!();
        fn populate_meta_dir (destination_dir : & Path , script_identity : u32) -> std :: io :: Result < PathBuf > { let meta_dir = destination_dir . join (META_DIR_NAME) ; std :: fs :: create_dir_all (& meta_dir) ? ; std :: fs :: write (meta_dir . join (META_IDENTITY) , format ! ("{}-{}" , script_identity , family_name ()) . as_bytes () ,) ? ; std :: fs :: write (meta_dir . join (META_GIT_VERSION) , std :: process :: Command :: new (GIT_PROGRAM) . arg ("--version") . output () ? . stdout ,) ? ; Ok (meta_dir) }
    };
}

populate_meta_dir!();