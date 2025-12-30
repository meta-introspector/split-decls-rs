// Generated macro for impl_1942 (impl)
macro_rules! Depcrate_key_log_fileimpl_1942 {
() => {
// Module: crate::key_log_file
// Provides: {"impl_1942"}
// Dependencies: {}
impl KeyLog for KeyLogFile { fn log (& self , label : & str , client_random : & [u8] , secret : & [u8]) { match self . 0 . lock () . unwrap () . try_write (label , client_random , secret) { Ok (()) => { } Err (e) => { warn ! ("error writing to key log file: {e}") ; } } } }
};
}
