// Generated macro for historic_transition_time (function)
macro_rules! Depcrate_parse_tzifhistoric_transition_time {
() => {
// Module: crate::parse::tzif
// Provides: {"historic_transition_time"}
// Dependencies: {}
# [doc = " A four- or eight-byte UNIX leap-time value."] # [doc = " Each value is used as a transition time at which the rules for"] # [doc = " computing local time may change. Each time value SHOULD be at least -2**59."] # [doc = ""] # [doc = " (-2**59 is the greatest negated power of 2 that predates the Big"] # [doc = " Bang, and avoiding earlier timestamps works around known `TZif`"] # [doc = " reader bugs relating to outlandishly negative timestamps.)"] fn historic_transition_time < const V : usize , Input > () -> impl Parser < Input , Output = Seconds > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { match V { 1 => be_i32 () . map (i64 :: from) . left () , _ => be_i64 () . right () , } . then (| time | { ensure (time , | & time | time >= (- 2_i64) . pow (59) , "transition time should not be less than -2.pow(59)" ,) }) . map (Seconds) }
};
}
