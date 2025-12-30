// Generated macro for AbiKind (enum)
macro_rules! Depcrate_callconv_aarch64AbiKind {
() => {
// Module: crate::callconv::aarch64
// Provides: {"AbiKind"}
// Dependencies: {}
# [doc = " Indicates the variant of the AArch64 ABI we are compiling for."] # [doc = " Used to accommodate Apple and Microsoft's deviations from the usual AAPCS ABI."] # [doc = ""] # [doc = " Corresponds to Clang's `AArch64ABIInfo::ABIKind`."] # [derive (Copy , Clone , PartialEq)] pub (crate) enum AbiKind { AAPCS , DarwinPCS , Win64 , }
};
}
