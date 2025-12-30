// Generated macro for impl_87 (impl)
macro_rules! Depcrate_parseimpl_87 {
() => {
// Module: crate::parse
// Provides: {"impl_87"}
// Dependencies: {}
impl FromStr for VersionReq { type Err = Error ; fn from_str (text : & str) -> Result < Self , Self :: Err > { let text = text . trim_start_matches (' ') ; if let Some ((ch , text)) = wildcard (text) { let rest = text . trim_start_matches (' ') ; if rest . is_empty () { return Ok (VersionReq :: STAR) ; } else if rest . starts_with (',') { return Err (Error :: new (ErrorKind :: WildcardNotTheOnlyComparator (ch))) ; } else { return Err (Error :: new (ErrorKind :: UnexpectedAfterWildcard)) ; } } let depth = 0 ; let mut comparators = Vec :: new () ; let len = version_req (text , & mut comparators , depth) ? ; unsafe { comparators . set_len (len) } Ok (VersionReq { comparators }) } }
};
}
