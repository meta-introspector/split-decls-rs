// Generated macro for Archive (struct)
macro_rules! Depcrate_archiveArchive {
() => {
// Module: crate::archive
// Provides: {"Archive"}
// Dependencies: {}
# [doc = " A top-level representation of an archive file."] # [doc = ""] # [doc = " This archive can have an entry added to it and it can be iterated over."] pub struct Archive < R : ? Sized + Read > { inner : ArchiveInner < R > , }
};
}
