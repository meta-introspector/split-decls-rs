// Generated macro for impl_1314 (impl)
macro_rules! Depcrate_windows_usersimpl_1314 {
() => {
// Module: crate::windows::users
// Provides: {"impl_1314"}
// Dependencies: {}
impl < T > NetApiBuffer < T > { pub fn inner_mut (& mut self) -> & mut * mut T { assert ! (self . 0 . is_null ()) ; & mut self . 0 } pub unsafe fn inner_mut_as_bytes (& mut self) -> & mut * mut u8 { unsafe { & mut * (self . inner_mut () as * mut * mut T as * mut * mut u8) } } }
};
}
