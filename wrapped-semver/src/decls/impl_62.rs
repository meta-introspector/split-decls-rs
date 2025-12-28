macro_rules! deps {
    () => {
        ErrorKind!();
        VersionReq!();
        Error!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl FromStr for VersionReq { type Err = Error ; fn from_str (text : & str) -> Result < Self , Self :: Err > { let text = text . trim_start_matches (' ') ; if let Some ((ch , text)) = wildcard (text) { let rest = text . trim_start_matches (' ') ; if rest . is_empty () { return Ok (VersionReq :: STAR) ; } else if rest . starts_with (',') { return Err (Error :: new (ErrorKind :: WildcardNotTheOnlyComparator (ch))) ; } else { return Err (Error :: new (ErrorKind :: UnexpectedAfterWildcard)) ; } } let depth = 0 ; let mut comparators = Vec :: new () ; let len = version_req (text , & mut comparators , depth) ? ; unsafe { comparators . set_len (len) } Ok (VersionReq { comparators }) } }
    };
}

impl_62!()