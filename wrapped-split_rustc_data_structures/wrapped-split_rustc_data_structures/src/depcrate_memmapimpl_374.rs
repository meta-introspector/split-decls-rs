// Generated macro for impl_374 (impl)
macro_rules! Depcrate_memmapimpl_374 {
() => {
// Module: crate::memmap
// Provides: {"impl_374"}
// Dependencies: {}
# [cfg (not (any (miri , target_arch = "wasm32")))] impl MmapMut { # [inline] pub fn map_anon (len : usize) -> io :: Result < Self > { let mmap = memmap2 :: MmapMut :: map_anon (len) ? ; Ok (MmapMut (mmap)) } # [inline] pub fn flush (& mut self) -> io :: Result < () > { self . 0 . flush () } # [inline] pub fn make_read_only (self) -> std :: io :: Result < Mmap > { let mmap = self . 0 . make_read_only () ? ; Ok (Mmap (mmap)) } }
};
}
