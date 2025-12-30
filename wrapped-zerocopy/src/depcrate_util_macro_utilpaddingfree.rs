// Generated macro for PaddingFree (trait)
macro_rules! Depcrate_util_macro_utilPaddingFree {
() => {
// Module: crate::util::macro_util
// Provides: {"PaddingFree"}
// Dependencies: {}
# [cfg_attr (zerocopy_diagnostic_on_unimplemented_1_78_0 , diagnostic :: on_unimplemented (message = "`{T}` has {PADDING_BYTES} total byte(s) of padding" , label = "types with padding cannot implement `IntoBytes`" , note = "consider using `zerocopy::Unalign` to lower the alignment of individual fields" , note = "consider adding explicit fields where padding would be" , note = "consider using `#[repr(packed)]` to remove padding"))] pub trait PaddingFree < T : ? Sized , const PADDING_BYTES : usize > { }
};
}
