// Generated macro for walk_many (function)
macro_rules! Depcrate_walkwalk_many {
() => {
// Module: crate::walk
// Provides: {"walk_many"}
// Dependencies: {}
pub fn walk_many (paths : & [& Path] , skip : impl Send + Sync + 'static + Fn (& Path , bool) -> bool , f : & mut dyn FnMut (& DirEntry , & str) ,) { let mut contents = Vec :: new () ; walk_no_read (paths , skip , & mut | entry | { contents . clear () ; let mut file = t ! (File :: open (entry . path ()) , entry . path ()) ; t ! (file . read_to_end (& mut contents) , entry . path ()) ; let contents_str = match std :: str :: from_utf8 (& contents) { Ok (s) => s , Err (_) => return , } ; f (entry , contents_str) ; }) ; }
};
}
