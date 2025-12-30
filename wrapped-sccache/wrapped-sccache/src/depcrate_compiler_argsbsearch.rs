// Generated macro for bsearch (function)
macro_rules! Depcrate_compiler_argsbsearch {
() => {
// Module: crate::compiler::args
// Provides: {"bsearch"}
// Dependencies: {}
# [doc = " Binary search for a `key` in a sorted array of items, given a comparison"] # [doc = " function. This implementation is tweaked to handle the case where the"] # [doc = " comparison function does prefix matching, where multiple items in the array"] # [doc = " might match, but the last match is the one actually matching."] fn bsearch < K , T , F > (key : K , items : & [T] , cmp : F) -> Option < & T > where F : Fn (& T , & K) -> Ordering , { let mut slice = items ; while ! slice . is_empty () { let middle = slice . len () / 2 ; match cmp (& slice [middle] , & key) { Ordering :: Equal => { let found_after = if slice . len () == 1 { None } else { bsearch (key , & slice [middle + 1 ..] , cmp) } ; return found_after . or (Some (& slice [middle])) ; } Ordering :: Greater => { slice = & slice [.. middle] ; } Ordering :: Less => { slice = & slice [middle + 1 ..] ; } } } None }
};
}
