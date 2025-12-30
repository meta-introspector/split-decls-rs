// Generated macro for impl_34 (impl)
macro_rules! Depcrateimpl_34 {
() => {
// Module: crate
// Provides: {"impl_34"}
// Dependencies: {}
impl < Z > Zeroize for Option < Z > where Z : Zeroize , { fn zeroize (& mut self) { if let Some (value) = self { value . zeroize () ; self . take () ; } unsafe { volatile_set ((self as * mut Self) . cast :: < u8 > () , 0 , size_of :: < Self > ()) ; } unsafe { ptr :: write_volatile (self , None) } atomic_fence () ; } }
};
}
