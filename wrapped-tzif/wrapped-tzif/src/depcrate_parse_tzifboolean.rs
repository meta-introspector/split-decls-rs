// Generated macro for boolean (function)
macro_rules! Depcrate_parse_tzifboolean {
() => {
// Module: crate::parse::tzif
// Provides: {"boolean"}
// Dependencies: {}
# [doc = " Parses a byte as a boolean value. The value must be exactly"] # [doc = " the numeric digit 0 (false) or the numeric digit 1 (true)."] fn boolean < Input > () -> impl Parser < Input , Output = bool > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { choice ((byte (b'\x00') . map (| _ | false) , byte (b'\x01') . map (| _ | true))) }
};
}
