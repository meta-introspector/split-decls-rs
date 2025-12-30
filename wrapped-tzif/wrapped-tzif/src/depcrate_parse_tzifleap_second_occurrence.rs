// Generated macro for leap_second_occurrence (function)
macro_rules! Depcrate_parse_tzifleap_second_occurrence {
() => {
// Module: crate::parse::tzif
// Provides: {"leap_second_occurrence"}
// Dependencies: {}
# [doc = " A four- or eight-byte UNIX leap time value specifying the time at which a leap-second"] # [doc = " correction occurs."] fn leap_second_occurrence < const V : usize , Input > () -> impl Parser < Input , Output = Seconds > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { match V { 1 => be_i32 () . map (i64 :: from) . left () , _ => be_i64 () . right () , } . map (Seconds) }
};
}
