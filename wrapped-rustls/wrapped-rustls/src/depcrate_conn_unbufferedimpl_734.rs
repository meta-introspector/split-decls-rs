// Generated macro for impl_734 (impl)
macro_rules! Depcrate_conn_unbufferedimpl_734 {
() => {
// Module: crate::conn::unbuffered
// Provides: {"impl_734"}
// Dependencies: {}
impl fmt :: Display for EncodeError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: InsufficientSize (InsufficientSizeError { required_size }) => write ! (f , "cannot encode due to insufficient size, {required_size} bytes are required") , Self :: AlreadyEncoded => "cannot encode, data has already been encoded" . fmt (f) , } } }
};
}
