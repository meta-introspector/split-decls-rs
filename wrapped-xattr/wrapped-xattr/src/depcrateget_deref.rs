// Generated macro for get_deref (function)
macro_rules! Depcrateget_deref {
() => {
// Module: crate
// Provides: {"get_deref"}
// Dependencies: {}
# [doc = " Get an extended attribute for the specified file (dereference symlinks)."] pub fn get_deref < N , P > (path : P , name : N) -> io :: Result < Option < Vec < u8 > > > where P : AsRef < Path > , N : AsRef < OsStr > , { util :: extract_noattr (sys :: get_path (path . as_ref () , name . as_ref () , true)) }
};
}
