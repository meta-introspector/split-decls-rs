// Generated macro for impl_67 (impl)
macro_rules! Depcrate_dataimpl_67 {
() => {
// Module: crate::data
// Provides: {"impl_67"}
// Dependencies: {}
impl DerefMut for StatVfs { fn deref_mut (& mut self) -> & mut [u8] { unsafe { slice :: from_raw_parts_mut (self as * mut StatVfs as * mut u8 , mem :: size_of :: < StatVfs > ()) } } }
};
}
