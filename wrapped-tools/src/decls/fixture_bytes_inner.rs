macro_rules! deps {
    () => {
        DirectoryRoot!();
    };
}

macro_rules! fixture_bytes_inner {
    () => {
        deps!();
        fn fixture_bytes_inner (path : impl AsRef < Path > , root : DirectoryRoot) -> Vec < u8 > { match std :: fs :: read (fixture_path_inner (path . as_ref () , root)) { Ok (res) => res , Err (_) => panic ! ("File at '{}' not found" , path . as_ref () . display ()) , } }
    };
}

fixture_bytes_inner!();