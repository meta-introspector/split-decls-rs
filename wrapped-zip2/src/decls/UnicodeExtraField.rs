macro_rules! UnicodeExtraField {
    () => {
        # [doc = " Info-ZIP Unicode Path Extra Field (0x7075) or Unicode Comment Extra Field (0x6375), as"] # [doc = " specified in APPNOTE 4.6.8 and 4.6.9"] # [derive (Clone , Debug)] pub struct UnicodeExtraField { crc32 : u32 , content : Box < [u8] > , }
    };
}

UnicodeExtraField!()