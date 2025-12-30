// Generated macro for impl_672 (impl)
macro_rules! Depcrate_ffi_os_strimpl_672 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_672"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl PartialOrd < str > for OsStr { # [inline] fn partial_cmp (& self , other : & str) -> Option < cmp :: Ordering > { self . partial_cmp (OsStr :: new (other)) } }
};
}
