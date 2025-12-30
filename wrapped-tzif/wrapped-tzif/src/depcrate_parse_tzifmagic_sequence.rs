// Generated macro for magic_sequence (function)
macro_rules! Depcrate_parse_tzifmagic_sequence {
() => {
// Module: crate::parse::tzif
// Provides: {"magic_sequence"}
// Dependencies: {}
# [doc = " Parses the four-byte ASCII \\[RFC20\\] sequence `\"TZif\"` (0x54 0x5A 0x69 0x42),"] # [doc = " which identifies the file as utilizing the Time Zone Information Format."] fn magic_sequence < Input > () -> impl Parser < Input , Output = u8 > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { byte (b'T') . with (byte (b'Z')) . with (byte (b'i')) . with (byte (b'f')) }
};
}
