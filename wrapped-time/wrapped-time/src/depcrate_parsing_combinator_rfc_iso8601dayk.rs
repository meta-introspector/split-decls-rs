// Generated macro for dayk (function)
macro_rules! Depcrate_parsing_combinator_rfc_iso8601dayk {
() => {
// Module: crate::parsing::combinator::rfc::iso8601
// Provides: {"dayk"}
// Dependencies: {}
# [doc = " Parse a day of the week."] # [inline] pub (crate) fn dayk (input : & [u8]) -> Option < ParsedItem < '_ , Weekday > > { first_match ([(b"1" . as_slice () , Weekday :: Monday) , (b"2" . as_slice () , Weekday :: Tuesday) , (b"3" . as_slice () , Weekday :: Wednesday) , (b"4" . as_slice () , Weekday :: Thursday) , (b"5" . as_slice () , Weekday :: Friday) , (b"6" . as_slice () , Weekday :: Saturday) , (b"7" . as_slice () , Weekday :: Sunday) ,] , true ,) (input) }
};
}
