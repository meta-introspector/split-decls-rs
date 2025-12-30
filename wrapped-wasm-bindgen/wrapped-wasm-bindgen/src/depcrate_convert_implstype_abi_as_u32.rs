// Generated macro for type_abi_as_u32 (macro)
macro_rules! Depcrate_convert_implstype_abi_as_u32 {
() => {
// Module: crate::convert::impls
// Provides: {"type_abi_as_u32"}
// Dependencies: {}
macro_rules ! type_abi_as_u32 { ($ ($ t : tt) *) => ($ (impl IntoWasmAbi for $ t { type Abi = u32 ; # [inline] fn into_abi (self) -> u32 { self as u32 } } impl FromWasmAbi for $ t { type Abi = u32 ; # [inline] unsafe fn from_abi (js : u32) -> Self { js as $ t } } impl OptionIntoWasmAbi for $ t { # [inline] fn none () -> u32 { U32_ABI_OPTION_SENTINEL } } impl OptionFromWasmAbi for $ t { # [inline] fn is_none (js : & u32) -> bool { * js == U32_ABI_OPTION_SENTINEL } }) *) }
};
}
