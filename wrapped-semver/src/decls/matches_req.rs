macro_rules! deps {
    () => {
        VersionReq!();
        Version!();
    };
}

macro_rules! matches_req {
    () => {
        deps!();
        pub (crate) fn matches_req (req : & VersionReq , ver : & Version) -> bool { for cmp in & req . comparators { if ! matches_impl (cmp , ver) { return false ; } } if ver . pre . is_empty () { return true ; } for cmp in & req . comparators { if pre_is_compatible (cmp , ver) { return true ; } } false }
    };
}

matches_req!();