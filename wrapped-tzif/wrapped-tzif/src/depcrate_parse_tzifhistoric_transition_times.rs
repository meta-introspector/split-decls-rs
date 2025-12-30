// Generated macro for historic_transition_times (function)
macro_rules! Depcrate_parse_tzifhistoric_transition_times {
() => {
// Module: crate::parse::tzif
// Provides: {"historic_transition_times"}
// Dependencies: {}
# [doc = " Parse a series of transition times sorted in strictly ascending order."] # [doc = " The number of time values is specified by the `timecnt`"] # [doc = " field in the header."] fn historic_transition_times < const V : usize , Input > (timecnt : usize ,) -> impl Parser < Input , Output = Vec < Seconds > > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { count_min_max (timecnt , timecnt , historic_transition_time :: < V , _ > ()) . then (| times : Vec < Seconds > | { ensure (times , | times | { times . iter () . zip (times . iter () . skip (1)) . all (| (lhs , rhs) | lhs <= rhs) } , "historic transition times should be in ascenting order" ,) } ,) }
};
}
