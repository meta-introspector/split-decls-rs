// Generated macro for parse_rug (function)
macro_rules! Depcrateparse_rug {
() => {
// Module: crate
// Provides: {"parse_rug"}
// Dependencies: {}
# [doc = " Try to parse the float type going via `rug`, for `f16` and `f128` which don't yet implement"] # [doc = " `FromStr`."] # [cfg (feature = "build-mpfr")] fn parse_rug < F > (input : & [& str] , idx : usize) -> F where F : libm_test :: Float + FromStrRadix , rug :: Float : az :: Cast < F > , { let s = input [idx] ; let msg = | | format ! ("invalid {} input '{s}'" , type_name ::< F > ()) ; if s . starts_with ("0x") { return F :: from_str_radix (s , 16) . unwrap_or_else (| _ | panic ! ("{}" , msg ())) ; } if s . starts_with ("0b") { return F :: from_str_radix (s , 2) . unwrap_or_else (| _ | panic ! ("{}" , msg ())) ; } let x = rug :: Float :: parse (s) . unwrap_or_else (| _ | panic ! ("{}" , msg ())) ; let x = rug :: Float :: with_val (F :: BITS , x) ; x . az () }
};
}
