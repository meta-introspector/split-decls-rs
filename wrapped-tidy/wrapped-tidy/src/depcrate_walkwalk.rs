// Generated macro for walk (function)
macro_rules! Depcrate_walkwalk {
() => {
// Module: crate::walk
// Provides: {"walk"}
// Dependencies: {}
pub fn walk (path : & Path , skip : impl Send + Sync + 'static + Fn (& Path , bool) -> bool , f : & mut dyn FnMut (& DirEntry , & str) ,) { walk_many (& [path] , skip , f) ; }
};
}
