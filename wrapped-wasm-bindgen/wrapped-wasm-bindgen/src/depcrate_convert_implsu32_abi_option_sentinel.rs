// Generated macro for U32_ABI_OPTION_SENTINEL (const)
macro_rules! Depcrate_convert_implsU32_ABI_OPTION_SENTINEL {
() => {
// Module: crate::convert::impls
// Provides: {"U32_ABI_OPTION_SENTINEL"}
// Dependencies: {}
# [doc = " The sentinel value is 0xFF_FFFF for primitives with less than 32 bits."] # [doc = ""] # [doc = " This value is used, so all small primitive types (`bool`, `i8`, `u8`,"] # [doc = " `i16`, `u16`, `char`) can use the same JS glue code. `char::MAX` is"] # [doc = " 0x10_FFFF btw."] const U32_ABI_OPTION_SENTINEL : u32 = 0x00FF_FFFFu32 ;
};
}
