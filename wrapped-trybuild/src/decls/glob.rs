macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! glob {
    () => {
        deps!();
        fn glob (pattern : & str) -> Result < Vec < PathBuf > > { let mut paths = glob :: glob (pattern) ? . map (| entry | entry . map_err (Error :: from)) . collect :: < Result < Vec < PathBuf > > > () ? ; paths . sort () ; Ok (paths) }
    };
}

glob!();