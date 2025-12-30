// Generated macro for impl_23 (impl)
macro_rules! Depcrate_dentimpl_23 {
() => {
// Module: crate::dent
// Provides: {"impl_23"}
// Dependencies: {}
impl Clone for DirEntry { # [cfg (windows)] fn clone (& self) -> DirEntry { DirEntry { path : self . path . clone () , ty : self . ty , follow_link : self . follow_link , depth : self . depth , metadata : self . metadata . clone () , } } # [cfg (unix)] fn clone (& self) -> DirEntry { DirEntry { path : self . path . clone () , ty : self . ty , follow_link : self . follow_link , depth : self . depth , ino : self . ino , } } # [cfg (not (any (unix , windows)))] fn clone (& self) -> DirEntry { DirEntry { path : self . path . clone () , ty : self . ty , follow_link : self . follow_link , depth : self . depth , } } }
};
}
