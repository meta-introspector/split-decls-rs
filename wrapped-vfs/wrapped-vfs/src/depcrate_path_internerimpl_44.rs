// Generated macro for impl_44 (impl)
macro_rules! Depcrate_path_internerimpl_44 {
() => {
// Module: crate::path_interner
// Provides: {"impl_44"}
// Dependencies: {}
impl PathInterner { # [doc = " Get the id corresponding to `path`."] # [doc = ""] # [doc = " If `path` does not exists in `self`, returns [`None`]."] pub (crate) fn get (& self , path : & VfsPath) -> Option < FileId > { self . map . get_index_of (path) . map (| i | FileId (i as u32)) } # [doc = " Insert `path` in `self`."] # [doc = ""] # [doc = " - If `path` already exists in `self`, returns its associated id;"] # [doc = " - Else, returns a newly allocated id."] pub (crate) fn intern (& mut self , path : VfsPath) -> FileId { let (id , _added) = self . map . insert_full (path) ; assert ! (id < FileId :: MAX as usize) ; FileId (id as u32) } # [doc = " Returns the path corresponding to `id`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `id` does not exists in `self`."] pub (crate) fn lookup (& self , id : FileId) -> & VfsPath { self . map . get_index (id . 0 as usize) . unwrap () } }
};
}
