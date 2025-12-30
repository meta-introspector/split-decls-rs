// Generated macro for transition_day_jn (function)
macro_rules! Depcrate_parse_posixtransition_day_jn {
() => {
// Module: crate::parse::posix
// Provides: {"transition_day_jn"}
// Dependencies: {}
# [doc = " Parses transition day specified by a leading `J`, e.g. `J15`."] # [doc = " This is a value in range `[1, 365]` in which the leap day is never considered."] fn transition_day_jn < Input > () -> impl Parser < Input , Output = TransitionDay > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { byte (b'J') . with (bounded_natural (1 , 365) . map (| natural | TransitionDay :: NoLeap (natural as u16))) }
};
}
