// Generated macro for DirEntry (struct)
macro_rules! Depcrate_direntDirEntry {
() => {
// Module: crate::dirent
// Provides: {"DirEntry"}
// Dependencies: {}
pub struct DirEntry < 'name > { pub inode : u64 , pub next_opaque_id : u64 , pub name : & 'name str , pub kind : DirentKind , }
};
}
