// Generated macro for impl_57 (impl)
macro_rules! Depcrate_kinda_sorta_danglingimpl_57 {
() => {
// Module: crate::kinda_sorta_dangling
// Provides: {"impl_57"}
// Dependencies: {}
impl < T : 'static > KindaSortaDangling < T > { # [inline] pub (crate) const fn new (dangle : T) -> Self { KindaSortaDangling { dangle : MaybeUninit :: new (dangle) , } } # [inline] pub (crate) fn into_inner (self) -> T { let manual = ManuallyDrop :: new (self) ; unsafe { manual . dangle . assume_init_read () } } }
};
}
