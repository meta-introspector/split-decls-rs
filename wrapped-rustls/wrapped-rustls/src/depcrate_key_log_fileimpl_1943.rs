// Generated macro for impl_1943 (impl)
macro_rules! Depcrate_key_log_fileimpl_1943 {
() => {
// Module: crate::key_log_file
// Provides: {"impl_1943"}
// Dependencies: {}
impl Debug for KeyLogFile { fn fmt (& self , f : & mut Formatter < '_ >) -> core :: fmt :: Result { match self . 0 . try_lock () { Ok (key_log_file) => write ! (f , "{key_log_file:?}") , Err (_) => write ! (f , "KeyLogFile {{ <locked> }}") , } } }
};
}
