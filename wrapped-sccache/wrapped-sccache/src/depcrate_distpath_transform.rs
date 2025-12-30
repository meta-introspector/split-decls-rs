// Generated macro for path_transform (module)
macro_rules! Depcrate_distpath_transform {
() => {
// Module: crate::dist
// Provides: {"path_transform"}
// Dependencies: {}
# [cfg (unix)] mod path_transform { use std :: iter ; use std :: path :: { Path , PathBuf } ; # [derive (Debug)] pub struct PathTransformer ; impl PathTransformer { pub fn new () -> Self { PathTransformer } pub fn as_dist_abs (& mut self , p : & Path) -> Option < String > { if ! p . is_absolute () { return None ; } self . as_dist (p) } pub fn as_dist (& mut self , p : & Path) -> Option < String > { p . as_os_str () . to_str () . map (Into :: into) } pub fn disk_mappings (& self) -> impl Iterator < Item = (PathBuf , String) > { iter :: empty () } pub fn to_local (& self , p : & str) -> Option < PathBuf > { Some (PathBuf :: from (p)) } } }
};
}
