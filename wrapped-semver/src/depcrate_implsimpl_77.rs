// Generated macro for impl_77 (impl)
macro_rules! Depcrate_implsimpl_77 {
() => {
// Module: crate::impls
// Provides: {"impl_77"}
// Dependencies: {}
impl Ord for BuildMetadata { fn cmp (& self , rhs : & Self) -> Ordering { if self . identifier . ptr_eq (& rhs . identifier) { return Ordering :: Equal ; } let lhs = self . as_str () . split ('.') ; let mut rhs = rhs . as_str () . split ('.') ; for lhs in lhs { let rhs = match rhs . next () { None => return Ordering :: Greater , Some (rhs) => rhs , } ; let is_ascii_digit = | b : u8 | b . is_ascii_digit () ; let ordering = match (lhs . bytes () . all (is_ascii_digit) , rhs . bytes () . all (is_ascii_digit) ,) { (true , true) => { let lhval = lhs . trim_start_matches ('0') ; let rhval = rhs . trim_start_matches ('0') ; Ord :: cmp (& lhval . len () , & rhval . len ()) . then_with (| | Ord :: cmp (lhval , rhval)) . then_with (| | Ord :: cmp (& lhs . len () , & rhs . len ())) } (true , false) => return Ordering :: Less , (false , true) => return Ordering :: Greater , (false , false) => Ord :: cmp (lhs , rhs) , } ; if ordering != Ordering :: Equal { return ordering ; } } if rhs . next () . is_none () { Ordering :: Equal } else { Ordering :: Less } } }
};
}
