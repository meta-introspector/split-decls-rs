// Generated macro for impl_67 (impl)
macro_rules! Depcrate_cp437impl_67 {
() => {
// Module: crate::cp437
// Provides: {"impl_67"}
// Dependencies: {}
impl FromCp437 for Box < [u8] > { type Target = Box < str > ; fn from_cp437 (self) -> Self :: Target { if self . iter () . all (| c | * c < 0x80) { String :: from_utf8 (self . into ()) . unwrap () } else { self . iter () . copied () . map (to_char) . collect () } . into_boxed_str () } }
};
}
