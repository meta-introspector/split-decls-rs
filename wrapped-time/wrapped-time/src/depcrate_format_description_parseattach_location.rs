// Generated macro for attach_location (function)
macro_rules! Depcrate_format_description_parseattach_location {
() => {
// Module: crate::format_description::parse
// Provides: {"attach_location"}
// Dependencies: {}
# [doc = " Attach [`Location`] information to each byte in the iterator."] # [inline] fn attach_location < 'item > (iter : impl Iterator < Item = & 'item u8 > ,) -> impl Iterator < Item = (& 'item u8 , Location) > { let mut byte_pos = 0 ; iter . map (move | byte | { let location = Location { byte : byte_pos } ; byte_pos += 1 ; (byte , location) }) }
};
}
