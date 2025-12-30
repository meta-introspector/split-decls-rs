// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
impl CertificateResult { # [doc = " Return the found certificates if no error occurred, otherwise panic."] # [track_caller] pub fn expect (self , msg : & str) -> Vec < CertificateDer < 'static > > { match self . errors . is_empty () { true => self . certs , false => panic ! ("{msg}: {:?}" , self . errors) , } } # [doc = " Return the found certificates if no error occurred, otherwise panic."] # [track_caller] pub fn unwrap (self) -> Vec < CertificateDer < 'static > > { match self . errors . is_empty () { true => self . certs , false => panic ! ("errors occurred while loading certificates: {:?}" , self . errors) , } } fn pem_error (& mut self , err : pem :: Error , path : & Path) { self . errors . push (Error { context : "failed to read PEM from file" , kind : match err { pem :: Error :: Io (err) => ErrorKind :: Io { inner : err , path : path . to_owned () , } , _ => ErrorKind :: Pem (err) , } , }) ; } fn io_error (& mut self , err : io :: Error , path : & Path , context : & 'static str) { self . errors . push (Error { context , kind : ErrorKind :: Io { inner : err , path : path . to_owned () , } , }) ; } # [cfg (any (windows , target_os = "macos"))] fn os_error (& mut self , err : Box < dyn StdError + Send + Sync + 'static > , context : & 'static str) { self . errors . push (Error { context , kind : ErrorKind :: Os (err) , }) ; } }
};
}
