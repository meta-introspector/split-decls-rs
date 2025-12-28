macro_rules! CanonicalPath {
    () => {
        # [derive (Eq , PartialEq , Ord , PartialOrd , Clone)] pub (crate) struct CanonicalPath (PathBuf) ;
    };
}

CanonicalPath!();