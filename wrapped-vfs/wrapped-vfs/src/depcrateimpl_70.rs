// Generated macro for impl_70 (impl)
macro_rules! Depcrateimpl_70 {
() => {
// Module: crate
// Provides: {"impl_70"}
// Dependencies: {}
impl FileId { const MAX : u32 = 0x7fff_ffff ; # [inline] pub const fn from_raw (raw : u32) -> FileId { assert ! (raw <= Self :: MAX) ; FileId (raw) } # [inline] pub const fn index (self) -> u32 { self . 0 } }
};
}
