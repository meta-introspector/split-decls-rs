// Generated macro for impl_649 (impl)
macro_rules! Depcrate_ule_optionimpl_649 {
() => {
// Module: crate::ule::option
// Provides: {"impl_649"}
// Dependencies: {}
impl < T : AsULE > AsULE for Option < T > { type ULE = OptionULE < T :: ULE > ; fn to_unaligned (self) -> OptionULE < T :: ULE > { OptionULE :: new (self . map (T :: to_unaligned)) } fn from_unaligned (other : OptionULE < T :: ULE >) -> Self { other . get () . map (T :: from_unaligned) } }
};
}
