// Generated macro for get (function)
macro_rules! Depcrateget {
() => {
// Module: crate
// Provides: {"get"}
// Dependencies: {}
# [doc = " Get an extended attribute for the specified file."] pub fn get < N , P > (path : P , name : N) -> io :: Result < Option < Vec < u8 > > > where P : AsRef < Path > , N : AsRef < OsStr > , { util :: extract_noattr (sys :: get_path (path . as_ref () , name . as_ref () , false)) }
};
}
