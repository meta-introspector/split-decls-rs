// Generated macro for first_match (function)
macro_rules! Depcrate_parsing_combinatorfirst_match {
() => {
// Module: crate::parsing::combinator
// Provides: {"first_match"}
// Dependencies: {}
# [doc = " Consume the first matching item, returning its associated value."] # [inline] pub (crate) fn first_match < 'a , T > (options : impl IntoIterator < Item = (& 'a [u8] , T) > , case_sensitive : bool ,) -> impl FnMut (& 'a [u8]) -> Option < ParsedItem < 'a , T > > { let mut options = options . into_iter () ; move | input | { options . find_map (| (expected , t) | { if case_sensitive { Some (ParsedItem (input . strip_prefix (expected) ? , t)) } else { let n = expected . len () ; if n <= input . len () { let (head , tail) = input . split_at (n) ; if head . eq_ignore_ascii_case (expected) { return Some (ParsedItem (tail , t)) ; } } None } }) } }
};
}
