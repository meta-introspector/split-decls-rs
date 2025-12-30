// Generated macro for copy_path_into_inner (function)
macro_rules! Depcrate_headercopy_path_into_inner {
() => {
// Module: crate::header
// Provides: {"copy_path_into_inner"}
// Dependencies: {}
fn copy_path_into_inner (mut slot : & mut [u8] , path : & Path , is_link_name : bool , is_truncated_gnu_long_path : bool ,) -> io :: Result < () > { let mut emitted = false ; let mut needs_slash = false ; let mut iter = path . components () . peekable () ; while let Some (component) = iter . next () { let bytes = path2bytes (Path :: new (component . as_os_str ())) ? ; match (component , is_link_name) { (Component :: Prefix (..) , false) | (Component :: RootDir , false) => { return Err (other ("paths in archives must be relative")) ; } (Component :: ParentDir , false) => { if ! is_truncated_gnu_long_path || iter . peek () . is_some () { return Err (other ("paths in archives must not have `..`")) ; } } (Component :: CurDir , false) if path . components () . count () == 1 => { } (Component :: CurDir , false) => continue , (Component :: Normal (_) , _) | (_ , true) => { } } ; if needs_slash { copy (& mut slot , b"/") ? ; } if bytes . contains (& b'/') { if let Component :: Normal (..) = component { return Err (other ("path component in archive cannot contain `/`")) ; } } copy (& mut slot , & bytes) ? ; if & * bytes != b"/" { needs_slash = true ; } emitted = true ; } if ! emitted { return Err (other ("paths in archives must have at least one component")) ; } if ends_with_slash (path) { copy (& mut slot , b"/") ? ; } return Ok (()) ; fn copy (slot : & mut & mut [u8] , bytes : & [u8]) -> io :: Result < () > { copy_into (slot , bytes) ? ; let tmp = mem :: take (slot) ; * slot = & mut tmp [bytes . len () ..] ; Ok (()) } }
};
}
