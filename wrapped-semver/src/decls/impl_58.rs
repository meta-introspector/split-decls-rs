macro_rules! deps {
    () => {
        Comparator!();
        VersionReq!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl FromIterator < Comparator > for VersionReq { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = Comparator > , { let comparators = Vec :: from_iter (iter) ; VersionReq { comparators } } }
    };
}

impl_58!()