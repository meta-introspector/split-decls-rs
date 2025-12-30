// Generated macro for transition_day_n (function)
macro_rules! Depcrate_parse_posixtransition_day_n {
() => {
// Module: crate::parse::posix
// Provides: {"transition_day_n"}
// Dependencies: {}
# [doc = " Parses transition day with no specified leading character, e.g. `15`."] # [doc = " This is a value in range `[0, 365]` in which the leap day is considered for leap years."] fn transition_day_n < Input > () -> impl Parser < Input , Output = TransitionDay > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { bounded_natural (0 , 365) . map (| natural | TransitionDay :: WithLeap (natural as u16)) }
};
}
