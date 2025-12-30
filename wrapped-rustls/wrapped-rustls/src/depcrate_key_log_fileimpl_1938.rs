// Generated macro for impl_1938 (impl)
macro_rules! Depcrate_key_log_fileimpl_1938 {
() => {
// Module: crate::key_log_file
// Provides: {"impl_1938"}
// Dependencies: {}
impl KeyLogFileInner { fn new (var : Option < OsString >) -> Self { let Some (path) = & var else { return Self { file : None , buf : Vec :: new () , } ; } ; let file = match OpenOptions :: new () . append (true) . create (true) . open (path) { Ok (f) => Some (f) , Err (e) => { warn ! ("unable to create key log file {path:?}: {e}") ; None } } ; Self { file , buf : Vec :: new () , } } fn try_write (& mut self , label : & str , client_random : & [u8] , secret : & [u8]) -> io :: Result < () > { let Some (file) = & mut self . file else { return Ok (()) ; } ; self . buf . truncate (0) ; write ! (self . buf , "{label} ") ? ; for b in client_random . iter () { write ! (self . buf , "{b:02x}") ? ; } write ! (self . buf , " ") ? ; for b in secret . iter () { write ! (self . buf , "{b:02x}") ? ; } writeln ! (self . buf) ? ; file . write_all (& self . buf) } }
};
}
