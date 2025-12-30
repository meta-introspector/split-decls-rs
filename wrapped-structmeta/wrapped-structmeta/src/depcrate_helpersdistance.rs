// Generated macro for distance (function)
macro_rules! Depcrate_helpersdistance {
() => {
// Module: crate::helpers
// Provides: {"distance"}
// Dependencies: {}
fn distance (s0 : & [char] , s1 : & [char]) -> Option < usize > { if s0 . len () > s1 . len () { return distance (s1 , s0) ; } if s0 . len () + 1 < s1 . len () { return None ; } let mut start = 0 ; while start < s0 . len () && start < s1 . len () && s0 [start] == s1 [start] { start += 1 ; } let mut end = 0 ; while start + end < s0 . len () && start + end < s1 . len () && s0 [s0 . len () - end - 1] == s1 [s1 . len () - end - 1] { end += 1 ; } if s0 . len () == s1 . len () { if start + end == s0 . len () { return Some (0) ; } if start + end + 1 == s0 . len () { return Some (1) ; } if start + end + 2 == s0 . len () && s0 [start] == s1 [start + 1] && s0 [start + 1] == s1 [start] { return Some (2) ; } } else if start + end == s0 . len () { return Some (1) ; } None }
};
}
