// Generated macro for walk (function)
macro_rules! Depcratewalk {
() => {
// Module: crate
// Provides: {"walk"}
// Dependencies: {}
fn walk (root : & Path , files : & mut Vec < (syn :: File , String) >) { for file in root . read_dir () . unwrap () { let file = file . unwrap () ; if file . file_type () . unwrap () . is_dir () { walk (& file . path () , files) ; continue ; } let path = file . path () ; if path . extension () . and_then (std :: ffi :: OsStr :: to_str) != Some ("rs") { continue ; } if path . file_name () . and_then (std :: ffi :: OsStr :: to_str) == Some ("test.rs") { continue ; } let mut contents = String :: new () ; File :: open (& path) . unwrap_or_else (| _ | panic ! ("can't open file at path: {}" , path . display ())) . read_to_string (& mut contents) . expect ("failed to read file to string") ; files . push ((syn :: parse_str :: < syn :: File > (& contents) . expect ("failed to parse") , path . display () . to_string () ,)) ; } }
};
}
