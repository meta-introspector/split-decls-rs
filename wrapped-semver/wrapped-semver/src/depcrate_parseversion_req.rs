// Generated macro for version_req (function)
macro_rules! Depcrate_parseversion_req {
() => {
// Module: crate::parse
// Provides: {"version_req"}
// Dependencies: {}
fn version_req (input : & str , out : & mut Vec < Comparator > , depth : usize) -> Result < usize , Error > { let (comparator , pos , text) = match comparator (input) { Ok (success) => success , Err (mut error) => { if let Some ((ch , mut rest)) = wildcard (input) { rest = rest . trim_start_matches (' ') ; if rest . is_empty () || rest . starts_with (',') { error . kind = ErrorKind :: WildcardNotTheOnlyComparator (ch) ; } } return Err (error) ; } } ; if text . is_empty () { out . reserve_exact (depth + 1) ; unsafe { out . as_mut_ptr () . add (depth) . write (comparator) } return Ok (depth + 1) ; } let text = if let Some (text) = text . strip_prefix (',') { text . trim_start_matches (' ') } else { let unexpected = text . chars () . next () . unwrap () ; return Err (Error :: new (ErrorKind :: ExpectedCommaFound (pos , unexpected))) ; } ; const MAX_COMPARATORS : usize = 32 ; if depth + 1 == MAX_COMPARATORS { return Err (Error :: new (ErrorKind :: ExcessiveComparators)) ; } let len = version_req (text , out , depth + 1) ? ; unsafe { out . as_mut_ptr () . add (depth) . write (comparator) } Ok (len) }
};
}
