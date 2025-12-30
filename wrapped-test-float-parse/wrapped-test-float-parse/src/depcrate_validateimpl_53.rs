// Generated macro for impl_53 (impl)
macro_rules! Depcrate_validateimpl_53 {
() => {
// Module: crate::validate
// Provides: {"impl_53"}
// Dependencies: {}
impl Rational { # [doc = " Turn a string into a rational. `None` if `NaN`."] fn parse (s : & str) -> Rational { let mut s = s ; if s . strip_prefix ('+') . unwrap_or (s) . eq_ignore_ascii_case ("nan") || s . eq_ignore_ascii_case ("-nan") { return Rational :: Nan ; } if s . strip_prefix ('+') . unwrap_or (s) . eq_ignore_ascii_case ("inf") { return Rational :: Inf ; } if s . eq_ignore_ascii_case ("-inf") { return Rational :: NegInf ; } if s . bytes () . all (| b | b . is_ascii_digit () || b == b'-') { return Rational :: Finite (BigRational :: from_str (s) . unwrap ()) ; } let mut ten_exp : i32 = 0 ; if let Some (pos) = s . bytes () . position (| b | b == b'e' || b == b'E') { let (dec , exp) = s . split_at (pos) ; s = dec ; ten_exp = exp [1 ..] . parse () . unwrap () ; } let mut s_owned ; if let Some (pos) = s . bytes () . position (| b | b == b'.') { ten_exp = ten_exp . checked_sub ((s . len () - pos - 1) . try_into () . unwrap ()) . unwrap () ; s_owned = s . to_owned () ; s_owned . remove (pos) ; s = & s_owned ; } let pow = POWERS_OF_TEN . get (& ten_exp) . unwrap_or_else (| | panic ! ("missing power of ten {ten_exp}")) ; let r = pow * BigInt :: from_str (s) . unwrap_or_else (| e | panic ! ("`BigInt::from_str(\"{s}\")` failed with {e}")) ; Rational :: Finite (r) } # [cfg (test)] fn expect_finite (self) -> BigRational { let Self :: Finite (r) = self else { panic ! ("got non rational: {self:?}") ; } ; r } }
};
}
