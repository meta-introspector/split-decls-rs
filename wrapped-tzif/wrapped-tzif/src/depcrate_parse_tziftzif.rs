// Generated macro for tzif (function)
macro_rules! Depcrate_parse_tziftzif {
() => {
// Module: crate::parse::tzif
// Provides: {"tzif"}
// Dependencies: {}
# [doc = " Parses a `TZif` binary file according to the following specification:"] # [doc = " <https://datatracker.ietf.org/doc/html/rfc8536>"] # [must_use] pub fn tzif < Input > () -> impl Parser < Input , Output = TzifData > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { header () . then (| header1 | { if header1 . version () == 1 { (value (header1) , data_block :: < 1 , _ > (header1) , value (None) . left () ,) } else { (value (header1) , data_block :: < 1 , _ > (header1) , header () . map (Some) . right () ,) } }) . then (| (header1 , block1 , header2) | match header2 { None => combine :: struct_parser ! { TzifData { header1 : value (header1) , data_block1 : value (block1) , header2 : value (header2) , data_block2 : value (None) , footer : value (None) , } } . left () , Some (header) => (match header . version () { 2 => combine :: struct_parser ! { TzifData { header1 : value (header1) , data_block1 : value (block1) , header2 : value (header2) , data_block2 : data_block ::< 2 , _ > (header) . map (Some) , footer : footer () . map (Some) , } } . left () , _ => combine :: struct_parser ! { TzifData { header1 : value (header1) , data_block1 : value (block1) , header2 : value (header2) , data_block2 : data_block ::< 3 , _ > (header) . map (Some) , footer : footer () . map (Some) , } } . right () , }) . right () , }) }
};
}
