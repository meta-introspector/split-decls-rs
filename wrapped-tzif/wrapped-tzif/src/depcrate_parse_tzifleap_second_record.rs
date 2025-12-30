// Generated macro for leap_second_record (function)
macro_rules! Depcrate_parse_tzifleap_second_record {
() => {
// Module: crate::parse::tzif
// Provides: {"leap_second_record"}
// Dependencies: {}
# [doc = " A series of eight- or twelve-byte records"] # [doc = " specifying the corrections that need to be applied to UTC in order"] # [doc = " to determine TAI. The records are sorted by the occurrence time"] # [doc = " in strictly ascending order. The number of records is specified"] # [doc = " by the \"leapcnt\" field in the header. Each record has one of the"] # [doc = " following structures (the lengths of multi-byte fields are shown"] # [doc = " in parentheses):"] # [doc = ""] # [doc = " > Version 1 Data Block:"] # [doc = " >"] # [doc = " > ```text"] # [doc = " > +---------------+---------------+"] # [doc = " > |  occur (4)    |  corr (4)     |"] # [doc = " > +---------------+---------------+"] # [doc = " > ```"] # [doc = " >"] # [doc = " > version-2+ Data Block:"] # [doc = " >"] # [doc = " > ```text"] # [doc = " > +---------------+---------------+---------------+"] # [doc = " > |  occur (8)                    |  corr (4)     |"] # [doc = " > +---------------+---------------+---------------+"] # [doc = " > ```"] fn leap_second_record < const V : usize , Input > () -> impl Parser < Input , Output = LeapSecondRecord > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { combine :: struct_parser ! { LeapSecondRecord { occurrence : leap_second_occurrence ::< V , _ > () , correction : leap_second_correction () , } } }
};
}
