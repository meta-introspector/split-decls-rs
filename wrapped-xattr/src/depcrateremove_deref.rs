// Generated macro for remove_deref (function)
macro_rules! Depcrateremove_deref {
() => {
// Module: crate
// Provides: {"remove_deref"}
// Dependencies: {}
# [doc = " Remove an extended attribute from the specified file (dereference symlinks)."] pub fn remove_deref < N , P > (path : P , name : N) -> io :: Result < () > where P : AsRef < Path > , N : AsRef < OsStr > , { sys :: remove_path (path . as_ref () , name . as_ref () , true) }
};
}
