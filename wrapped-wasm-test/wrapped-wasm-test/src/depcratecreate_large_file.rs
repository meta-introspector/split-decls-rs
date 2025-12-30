// Generated macro for create_large_file (function)
macro_rules! Depcratecreate_large_file {
() => {
// Module: crate
// Provides: {"create_large_file"}
// Dependencies: {}
pub fn create_large_file () { let mut file = File :: create (FILE_PATH) . expect ("Could not create file") ; let buffer = vec ! [0u8 ; FILE_SIZE as usize] ; file . write_all (& buffer) . expect ("Could not write to file") ; }
};
}
