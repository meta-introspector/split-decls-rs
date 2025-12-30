// Generated macro for walk_dir (function)
macro_rules! Depcrate_walkwalk_dir {
() => {
// Module: crate::walk
// Provides: {"walk_dir"}
// Dependencies: {}
pub (crate) fn walk_dir (path : & Path , skip : impl Send + Sync + 'static + Fn (& Path) -> bool , f : & mut dyn FnMut (& DirEntry) ,) { let mut walker = ignore :: WalkBuilder :: new (path) ; let walker = walker . filter_entry (move | e | ! skip (e . path ())) ; for entry in walker . build () . flatten () { if entry . path () . is_dir () { f (& entry) ; } } }
};
}
