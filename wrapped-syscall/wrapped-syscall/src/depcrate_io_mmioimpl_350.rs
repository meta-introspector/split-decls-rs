// Generated macro for impl_350 (impl)
macro_rules! Depcrate_io_mmioimpl_350 {
() => {
// Module: crate::io::mmio
// Provides: {"impl_350"}
// Dependencies: {}
impl < T > Mmio < T > { pub unsafe fn zeroed () -> Self { Self { value : MaybeUninit :: zeroed () , } } pub unsafe fn uninit () -> Self { Self { value : MaybeUninit :: uninit () , } } pub const fn from (value : T) -> Self { Self { value : MaybeUninit :: new (value) , } } }
};
}
