// Generated macro for transition_date (function)
macro_rules! Depcrate_parse_posixtransition_date {
() => {
// Module: crate::parse::posix
// Provides: {"transition_date"}
// Dependencies: {}
# [doc = " Parses a DST transition date including the transition day and the transition time in seconds."] # [doc = ""] # [doc = " See [`transition_day`] and [`transition_time`] for more information."] fn transition_date < Input > () -> impl Parser < Input , Output = TransitionDate > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { combine :: struct_parser ! { TransitionDate { day : transition_day () , time : transition_time () , } } }
};
}
