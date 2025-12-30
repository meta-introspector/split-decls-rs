// Generated macro for dst_transition_info (function)
macro_rules! Depcrate_parse_posixdst_transition_info {
() => {
// Module: crate::parse::posix
// Provides: {"dst_transition_info"}
// Dependencies: {}
# [doc = " Parses DST transition information including the variant info, and the transition dates."] # [doc = ""] # [doc = " See [`dst_variant_info`], [`transition_date`] for more information."] fn dst_transition_info < Input > (std_offset : Seconds) -> impl Parser < Input , Output = DstTransitionInfo > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { combine :: struct_parser ! { DstTransitionInfo { variant_info : dst_variant_info (std_offset) , start_date : byte (b',') . with (transition_date ()) , end_date : byte (b',') . with (transition_date ()) , } } }
};
}
