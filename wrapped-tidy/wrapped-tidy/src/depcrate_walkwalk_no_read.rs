// Generated macro for walk_no_read (function)
macro_rules! Depcrate_walkwalk_no_read {
() => {
// Module: crate::walk
// Provides: {"walk_no_read"}
// Dependencies: {}
pub (crate) fn walk_no_read (paths : & [& Path] , skip : impl Send + Sync + 'static + Fn (& Path , bool) -> bool , f : & mut dyn FnMut (& DirEntry) ,) { let mut walker = ignore :: WalkBuilder :: new (paths [0]) ; for path in & paths [1 ..] { walker . add (path) ; } let walker = walker . filter_entry (move | e | { ! skip (e . path () , e . file_type () . map (| ft | ft . is_dir ()) . unwrap_or (false)) }) ; for entry in walker . build () . flatten () { if entry . file_type () . is_none_or (| kind | kind . is_dir () || kind . is_symlink ()) { continue ; } f (& entry) ; } }
};
}
