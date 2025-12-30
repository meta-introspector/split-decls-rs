// Generated macro for DynamicPaddingFree (trait)
macro_rules! Depcrate_util_macro_utilDynamicPaddingFree {
() => {
// Module: crate::util::macro_util
// Provides: {"DynamicPaddingFree"}
// Dependencies: {}
# [cfg_attr (zerocopy_diagnostic_on_unimplemented_1_78_0 , diagnostic :: on_unimplemented (message = "`{T}` has one or more padding bytes" , label = "types with padding cannot implement `IntoBytes`" , note = "consider using `zerocopy::Unalign` to lower the alignment of individual fields" , note = "consider adding explicit fields where padding would be" , note = "consider using `#[repr(packed)]` to remove padding"))] pub trait DynamicPaddingFree < T : ? Sized , const HAS_PADDING : bool > { }
};
}
