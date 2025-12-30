// Generated macro for year (function)
macro_rules! Depcrate_parsing_combinator_rfc_iso8601year {
() => {
// Module: crate::parsing::combinator::rfc::iso8601
// Provides: {"year"}
// Dependencies: {}
# [doc = " Parse a possibly expanded year."] # [inline] pub (crate) fn year (input : & [u8]) -> Option < ParsedItem < '_ , i32 > > { Some (match sign (input) { Some (ParsedItem (input , sign)) => exactly_n_digits :: < 6 , u32 > (input) ? . map (| val | { let val = val . cast_signed () ; if sign == b'-' { - val } else { val } }) , None => exactly_n_digits :: < 4 , u32 > (input) ? . map (| val | val . cast_signed ()) , }) }
};
}
