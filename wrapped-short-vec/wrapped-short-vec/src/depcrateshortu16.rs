// Generated macro for ShortU16 (struct)
macro_rules! DepcrateShortU16 {
() => {
// Module: crate
// Provides: {"ShortU16"}
// Dependencies: {}
# [doc = " Same as u16, but serialized with 1 to 3 bytes. If the value is above"] # [doc = " 0x7f, the top bit is set and the remaining value is stored in the next"] # [doc = " bytes. Each byte follows the same pattern until the 3rd byte. The 3rd"] # [doc = " byte may only have the 2 least-significant bits set, otherwise the encoded"] # [doc = " value will overflow the u16."] # [cfg_attr (feature = "frozen-abi" , derive (AbiExample))] pub struct ShortU16 (pub u16) ;
};
}
