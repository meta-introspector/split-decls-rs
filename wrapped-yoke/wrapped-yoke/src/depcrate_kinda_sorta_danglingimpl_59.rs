// Generated macro for impl_59 (impl)
macro_rules! Depcrate_kinda_sorta_danglingimpl_59 {
() => {
// Module: crate::kinda_sorta_dangling
// Provides: {"impl_59"}
// Dependencies: {}
impl < T : 'static > DerefMut for KindaSortaDangling < T > { # [inline] fn deref_mut (& mut self) -> & mut T { unsafe { self . dangle . assume_init_mut () } } }
};
}
