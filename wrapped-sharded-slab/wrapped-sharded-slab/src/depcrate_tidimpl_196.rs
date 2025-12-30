// Generated macro for impl_196 (impl)
macro_rules! Depcrate_tidimpl_196 {
() => {
// Module: crate::tid
// Provides: {"impl_196"}
// Dependencies: {}
impl < C > Tid < C > { # [cold] fn poisoned () -> Self { Self { id : std :: usize :: MAX , _not_send : PhantomData , _cfg : PhantomData , } } # [doc = " Returns true if the local thread ID was accessed while unwinding."] pub (crate) fn is_poisoned (& self) -> bool { self . id == std :: usize :: MAX } }
};
}
