// Generated macro for impl_58 (impl)
macro_rules! Depcrate_kinda_sorta_danglingimpl_58 {
() => {
// Module: crate::kinda_sorta_dangling
// Provides: {"impl_58"}
// Dependencies: {}
impl < T : 'static > Deref for KindaSortaDangling < T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { self . dangle . assume_init_ref () } } }
};
}
