// Generated macro for impl_100 (impl)
macro_rules! Depcrate_direntimpl_100 {
() => {
// Module: crate::dirent
// Provides: {"impl_100"}
// Dependencies: {}
impl DirentKind { pub fn try_from_raw (raw : u8) -> Option < Self > { Some (match raw { 0 => Self :: Unspecified , 2 => Self :: CharDev , 4 => Self :: Directory , 6 => Self :: BlockDev , 8 => Self :: Regular , 10 => Self :: Symlink , 12 => Self :: Socket , _ => return None , }) } }
};
}
