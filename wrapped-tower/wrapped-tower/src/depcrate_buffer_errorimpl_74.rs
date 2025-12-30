// Generated macro for impl_74 (impl)
macro_rules! Depcrate_buffer_errorimpl_74 {
() => {
// Module: crate::buffer::error
// Provides: {"impl_74"}
// Dependencies: {}
impl fmt :: Display for ServiceError { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { write ! (fmt , "buffered service failed: {}" , self . inner) } }
};
}
