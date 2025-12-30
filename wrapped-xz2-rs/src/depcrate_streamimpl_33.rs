// Generated macro for impl_33 (impl)
macro_rules! Depcrate_streamimpl_33 {
() => {
// Module: crate::stream
// Provides: {"impl_33"}
// Dependencies: {}
impl From < Error > for io :: Error { fn from (e : Error) -> io :: Error { let kind = match e { Error :: Data => std :: io :: ErrorKind :: InvalidData , Error :: Options => std :: io :: ErrorKind :: InvalidInput , Error :: Format => std :: io :: ErrorKind :: InvalidData , Error :: MemLimit => std :: io :: ErrorKind :: Other , Error :: Mem => std :: io :: ErrorKind :: Other , Error :: Program => std :: io :: ErrorKind :: Other , Error :: NoCheck => std :: io :: ErrorKind :: InvalidInput , Error :: UnsupportedCheck => std :: io :: ErrorKind :: Other , } ; io :: Error :: new (kind , e) } }
};
}
