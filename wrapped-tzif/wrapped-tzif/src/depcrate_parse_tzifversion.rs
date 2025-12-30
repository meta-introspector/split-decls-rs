// Generated macro for version (function)
macro_rules! Depcrate_parse_tzifversion {
() => {
// Module: crate::parse::tzif
// Provides: {"version"}
// Dependencies: {}
# [doc = " Parse the `TZif` version number specified by <https://datatracker.ietf.org/doc/html/rfc8536>"] # [doc = " > A byte identifying the version of the file's format."] # [doc = " > The value MUST be one of the following:"] # [doc = " >"] # [doc = " > NUL (0x00)  Version 1"] # [doc = " >"] # [doc = " > '2' (0x32)  Version 2"] # [doc = " >"] # [doc = " > '3' (0x33)  Version 3"] fn version < Input > () -> impl Parser < Input , Output = usize > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { one_of ([0 , b'2' , b'3']) . map (| byte : u8 | byte . saturating_sub (b'0') as usize) . map (| version | if version == 0 { 1 } else { version }) }
};
}
