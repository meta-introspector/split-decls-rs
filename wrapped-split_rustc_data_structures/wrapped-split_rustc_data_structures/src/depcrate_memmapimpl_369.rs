// Generated macro for impl_369 (impl)
macro_rules! Depcrate_memmapimpl_369 {
() => {
// Module: crate::memmap
// Provides: {"impl_369"}
// Dependencies: {}
# [cfg (any (miri , target_arch = "wasm32"))] impl Mmap { # [inline] pub unsafe fn map (mut file : File) -> io :: Result < Self > { use std :: io :: Read ; let mut data = Vec :: new () ; file . read_to_end (& mut data) ? ; Ok (Mmap (data)) } }
};
}
