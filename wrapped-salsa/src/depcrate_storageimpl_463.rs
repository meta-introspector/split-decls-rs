// Generated macro for impl_463 (impl)
macro_rules! Depcrate_storageimpl_463 {
() => {
// Module: crate::storage
// Provides: {"impl_463"}
// Dependencies: {}
# [allow (clippy :: undocumented_unsafe_blocks)] unsafe impl < T : HasStorage > ZalsaDatabase for T { # [inline (always)] fn zalsa (& self) -> & Zalsa { & self . storage () . handle . zalsa_impl } fn zalsa_mut (& mut self) -> & mut Zalsa { self . storage_mut () . cancel_others () } # [inline (always)] fn zalsa_local (& self) -> & ZalsaLocal { & self . storage () . zalsa_local } # [inline (always)] fn fork_db (& self) -> RawDatabase < 'static > { Box :: leak (Box :: new (self . clone ())) . into () } }
};
}
