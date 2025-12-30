// Generated macro for impl_487 (impl)
macro_rules! Depcrate_unix_apple_macos_utilsimpl_487 {
() => {
// Module: crate::unix::apple::macos::utils
// Provides: {"impl_487"}
// Dependencies: {}
impl IOReleaser { pub (crate) fn new (obj : u32) -> Option < Self > { IoObject :: new (obj) . map (Self) } # [cfg (feature = "disk")] pub (crate) unsafe fn new_unchecked (obj : u32) -> Self { debug_assert_ne ! (obj , 0) ; unsafe { Self (IoObject :: new_unchecked (obj)) } } # [inline] pub (crate) fn inner (& self) -> u32 { self . 0 . get () } }
};
}
