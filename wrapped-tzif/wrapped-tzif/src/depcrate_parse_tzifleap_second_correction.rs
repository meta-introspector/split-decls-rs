// Generated macro for leap_second_correction (function)
macro_rules! Depcrate_parse_tzifleap_second_correction {
() => {
// Module: crate::parse::tzif
// Provides: {"leap_second_correction"}
// Dependencies: {}
# [doc = " A four-byte signed integer specifying the value of LEAPCORR on or after the"] # [doc = " occurrence. The correction value in the first leap-second record, if present,"] # [doc = " MUST be either one (1) or minus one (-1)."] fn leap_second_correction < Input > () -> impl Parser < Input , Output = i32 > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { be_i32 () }
};
}
