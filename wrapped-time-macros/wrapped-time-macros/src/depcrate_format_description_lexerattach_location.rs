// Generated macro for attach_location (function)
macro_rules! Depcrate_format_description_lexerattach_location {
() => {
// Module: crate::format_description::lexer
// Provides: {"attach_location"}
// Dependencies: {}
fn attach_location < 'item > (iter : impl Iterator < Item = & 'item u8 > , proc_span : proc_macro :: Span ,) -> impl Iterator < Item = (& 'item u8 , Location) > { let mut byte_pos = 0 ; iter . map (move | byte | { let location = Location { byte : byte_pos , proc_span , } ; byte_pos += 1 ; (byte , location) }) }
};
}
