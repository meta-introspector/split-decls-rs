macro_rules! deps {
    () => {
        CanonicalPath!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl CanonicalPath { pub (crate) fn new (path : & Path) -> Self { if let Ok (canonical) = path . canonicalize () { CanonicalPath (canonical) } else { CanonicalPath (path . to_owned ()) } } }
    };
}

impl_15!()