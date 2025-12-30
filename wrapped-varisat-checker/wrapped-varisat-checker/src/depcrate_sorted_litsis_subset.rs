// Generated macro for is_subset (function)
macro_rules! Depcrate_sorted_litsis_subset {
() => {
// Module: crate::sorted_lits
// Provides: {"is_subset"}
// Dependencies: {}
# [doc = " Test whether a set of literals is a (strict) subset of another set of literals"] # [doc = ""] # [doc = " Requires subset and superset to be sorted."] pub fn is_subset (mut subset : & [Lit] , mut superset : & [Lit] , strict : bool) -> bool { let mut is_strict = ! strict ; while let Some ((& sub_min , sub_rest)) = subset . split_first () { if let Some ((& super_min , super_rest)) = superset . split_first () { match sub_min . cmp (& super_min) { Ordering :: Less => { return false ; } Ordering :: Greater => { superset = super_rest ; is_strict = true ; } Ordering :: Equal => { superset = super_rest ; subset = sub_rest ; } } } else { return false ; } } is_strict |= ! superset . is_empty () ; is_strict }
};
}
