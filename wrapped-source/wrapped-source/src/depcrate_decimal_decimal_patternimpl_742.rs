// Generated macro for impl_742 (impl)
macro_rules! Depcrate_decimal_decimal_patternimpl_742 {
() => {
// Module: crate::decimal::decimal_pattern
// Provides: {"impl_742"}
// Dependencies: {}
impl FromStr for DecimalPattern { type Err = Error ; fn from_str (pattern : & str) -> Result < Self , Self :: Err > { let (positive , negative) = match pattern . split (';') . next_tuple () { Some ((u , s)) => (u . parse () ? , Some (s . parse () ?)) , None => (pattern . parse () ? , None) , } ; Ok (Self { positive , negative }) } }
};
}
