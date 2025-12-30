// Generated macro for parse (function)
macro_rules! Depcrateparse {
() => {
// Module: crate
// Provides: {"parse"}
// Dependencies: {}
# [doc = " Try to parse the number, printing a nice message on failure."] fn parse < T : FromStr + FromStrRadix > (input : & [& str] , idx : usize) -> T { let s = input [idx] ; let msg = | | format ! ("invalid {} input '{s}'" , type_name ::< T > ()) ; if s . starts_with ("0x") || s . starts_with ("-0x") { return T :: from_str_radix (s , 16) . unwrap_or_else (| _ | panic ! ("{}" , msg ())) ; } if s . starts_with ("0b") { return T :: from_str_radix (s , 2) . unwrap_or_else (| _ | panic ! ("{}" , msg ())) ; } s . parse () . unwrap_or_else (| _ | panic ! ("{}" , msg ())) }
};
}
