// Generated macro for read_large_file (function)
macro_rules! Depcrateread_large_file {
() => {
// Module: crate
// Provides: {"read_large_file"}
// Dependencies: {}
pub fn read_large_file () { let mut file = File :: open (FILE_PATH) . expect ("Could not open file") ; let mut buffer = Vec :: new () ; file . read_to_end (& mut buffer) . expect ("Could not read file") ; }
};
}
