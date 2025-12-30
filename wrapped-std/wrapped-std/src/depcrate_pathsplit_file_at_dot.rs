// Generated macro for split_file_at_dot (function)
macro_rules! Depcrate_pathsplit_file_at_dot {
() => {
// Module: crate::path
// Provides: {"split_file_at_dot"}
// Dependencies: {}
fn split_file_at_dot (file : & OsStr) -> (& OsStr , Option < & OsStr >) { let slice = file . as_encoded_bytes () ; if slice == b".." { return (file , None) ; } let i = match slice [1 ..] . iter () . position (| b | * b == b'.') { Some (i) => i + 1 , None => return (file , None) , } ; let before = & slice [.. i] ; let after = & slice [i + 1 ..] ; unsafe { (OsStr :: from_encoded_bytes_unchecked (before) , Some (OsStr :: from_encoded_bytes_unchecked (after)) ,) } }
};
}
