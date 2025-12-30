// Generated macro for transition_day (function)
macro_rules! Depcrate_parse_posixtransition_day {
() => {
// Module: crate::parse::posix
// Provides: {"transition_day"}
// Dependencies: {}
# [doc = " Parses a transition day in any of three unambiguous formats."] # [doc = ""] # [doc = " See [`transition_day_mwd`], [`transition_day_jn`], and [`transition_day_n`]"] # [doc = " for more information."] fn transition_day < Input > () -> impl Parser < Input , Output = TransitionDay > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { choice ((transition_day_mwd () , transition_day_jn () , transition_day_n () ,)) }
};
}
