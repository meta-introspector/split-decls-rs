// Generated macro for impl_73 (impl)
macro_rules! Depcrate_commonimpl_73 {
() => {
// Module: crate::common
// Provides: {"impl_73"}
// Dependencies: {}
impl < T : Unpin > SyncWriteAdapter < '_ , '_ , T > { # [inline] fn poll_with < U > (& mut self , f : impl FnOnce (Pin < & mut T > , & mut Context < '_ >) -> Poll < io :: Result < U > > ,) -> io :: Result < U > { match f (Pin :: new (self . io) , self . cx) { Poll :: Ready (result) => result , Poll :: Pending => Err (io :: ErrorKind :: WouldBlock . into ()) , } } }
};
}
