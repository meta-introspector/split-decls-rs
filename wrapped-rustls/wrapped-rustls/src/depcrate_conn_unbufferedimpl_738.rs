// Generated macro for impl_738 (impl)
macro_rules! Depcrate_conn_unbufferedimpl_738 {
() => {
// Module: crate::conn::unbuffered
// Provides: {"impl_738"}
// Dependencies: {}
impl fmt :: Display for EncryptError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: InsufficientSize (InsufficientSizeError { required_size }) => write ! (f , "cannot encrypt due to insufficient size, {required_size} bytes are required") , Self :: EncryptExhausted => f . write_str ("encrypter has been exhausted") , } } }
};
}
