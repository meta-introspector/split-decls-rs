// Generated macro for ArchiveInner (struct)
macro_rules! Depcrate_archiveArchiveInner {
() => {
// Module: crate::archive
// Provides: {"ArchiveInner"}
// Dependencies: {}
pub struct ArchiveInner < R : ? Sized > { pos : Cell < u64 > , mask : u32 , unpack_xattrs : bool , preserve_permissions : bool , preserve_ownerships : bool , preserve_mtime : bool , overwrite : bool , ignore_zeros : bool , obj : RefCell < R > , }
};
}
