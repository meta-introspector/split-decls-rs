// Generated macro for impl_375 (impl)
macro_rules! Depcrate_memmapimpl_375 {
() => {
// Module: crate::memmap
// Provides: {"impl_375"}
// Dependencies: {}
# [cfg (any (miri , target_arch = "wasm32"))] impl MmapMut { # [inline] pub fn map_anon (len : usize) -> io :: Result < Self > { let data = Vec :: with_capacity (len) ; Ok (MmapMut (data)) } # [inline] pub fn flush (& mut self) -> io :: Result < () > { Ok (()) } # [inline] pub fn make_read_only (self) -> std :: io :: Result < Mmap > { Ok (Mmap (self . 0)) } }
};
}
