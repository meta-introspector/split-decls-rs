// Generated macro for set (function)
macro_rules! Depcrateset {
() => {
// Module: crate
// Provides: {"set"}
// Dependencies: {}
# [doc = " Set an extended attribute on the specified file."] pub fn set < N , P > (path : P , name : N , value : & [u8]) -> io :: Result < () > where P : AsRef < Path > , N : AsRef < OsStr > , { sys :: set_path (path . as_ref () , name . as_ref () , value , false) }
};
}
