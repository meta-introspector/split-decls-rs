// Generated macro for impl_125 (impl)
macro_rules! Depcrateimpl_125 {
() => {
// Module: crate
// Provides: {"impl_125"}
// Dependencies: {}
impl Ancestor { # [doc = " Create a new ancestor from the given directory path."] # [cfg (windows)] fn new (dent : & DirEntry) -> io :: Result < Ancestor > { let handle = Handle :: from_path (dent . path ()) ? ; Ok (Ancestor { path : dent . path () . to_path_buf () , handle }) } # [doc = " Create a new ancestor from the given directory path."] # [cfg (not (windows))] fn new (dent : & DirEntry) -> io :: Result < Ancestor > { Ok (Ancestor { path : dent . path () . to_path_buf () }) } # [doc = " Returns true if and only if the given open file handle corresponds to"] # [doc = " the same directory as this ancestor."] # [cfg (windows)] fn is_same (& self , child : & Handle) -> io :: Result < bool > { Ok (child == & self . handle) } # [doc = " Returns true if and only if the given open file handle corresponds to"] # [doc = " the same directory as this ancestor."] # [cfg (not (windows))] fn is_same (& self , child : & Handle) -> io :: Result < bool > { Ok (child == & Handle :: from_path (& self . path) ?) } }
};
}
