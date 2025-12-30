// Generated macro for impl_60 (impl)
macro_rules! Depcrate_kinda_sorta_danglingimpl_60 {
() => {
// Module: crate::kinda_sorta_dangling
// Provides: {"impl_60"}
// Dependencies: {}
impl < T : 'static > Drop for KindaSortaDangling < T > { # [inline] fn drop (& mut self) { unsafe { self . dangle . as_mut_ptr () . drop_in_place () ; } } }
};
}
