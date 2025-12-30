// Generated macro for UnicodeExtraField (struct)
macro_rules! Depcrate_extra_fields_zipinfo_utf8UnicodeExtraField {
() => {
// Module: crate::extra_fields::zipinfo_utf8
// Provides: {"UnicodeExtraField"}
// Dependencies: {}
# [doc = " Info-ZIP Unicode Path Extra Field (0x7075) or Unicode Comment Extra Field (0x6375), as"] # [doc = " specified in APPNOTE 4.6.8 and 4.6.9"] # [derive (Clone , Debug)] pub struct UnicodeExtraField { crc32 : u32 , content : Box < [u8] > , }
};
}
