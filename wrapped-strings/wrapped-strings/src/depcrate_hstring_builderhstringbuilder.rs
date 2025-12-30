// Generated macro for HStringBuilder (struct)
macro_rules! Depcrate_hstring_builderHStringBuilder {
() => {
// Module: crate::hstring_builder
// Provides: {"HStringBuilder"}
// Dependencies: {}
# [doc = " An [HSTRING] builder that supports preallocating the `HSTRING` to avoid extra allocations and copies."] # [doc = ""] # [doc = " This is similar to the `WindowsPreallocateStringBuffer` function but implemented directly in Rust for efficiency."] # [doc = " It is implemented as a separate type since [HSTRING] values are immutable."] pub struct HStringBuilder (* mut HStringHeader) ;
};
}
