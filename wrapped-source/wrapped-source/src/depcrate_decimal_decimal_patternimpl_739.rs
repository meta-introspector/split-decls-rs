// Generated macro for impl_739 (impl)
macro_rules! Depcrate_decimal_decimal_patternimpl_739 {
() => {
// Module: crate::decimal::decimal_pattern
// Provides: {"impl_739"}
// Dependencies: {}
impl FromStr for DecimalSubPattern { type Err = Error ; fn from_str (subpattern : & str) -> Result < Self , Self :: Err > { let i = subpattern . find (['#' , '0' , ',' , '.']) ; let i = match i { Some (i) => i , None => return Err (Error :: NoBodyInSubpattern) , } ; let j = subpattern [i ..] . find (| c : char | ! matches ! (c , '#' | '0' | ',' | '.')) . unwrap_or (subpattern . len () - i) + i ; let prefix = & subpattern [.. i] ; let body = & subpattern [i .. j] ; let suffix = & subpattern [j ..] ; let (primary_grouping , secondary_grouping , min_fraction_digits , max_fraction_digits) = match body { "#,##0.###" => (3 , 3 , 0 , 3) , "#,##,##0.###" => (3 , 2 , 0 , 3) , "0.######" => (0 , 0 , 0 , 6) , "#,##0.00" => (3 , 3 , 2 , 2) , "#,#0.###" => (2 , 2 , 0 , 3) , "#,##,##0.00" => (3 , 2 , 2 , 2) , "#,#0.00" => (2 , 2 , 2 , 2) , _ => return Err (Error :: UnknownPatternBody (body . to_string ())) , } ; Ok (Self { prefix : prefix . into () , suffix : suffix . into () , primary_grouping , secondary_grouping , min_fraction_digits , max_fraction_digits , }) } }
};
}
