// Generated macro for impl_53 (impl)
macro_rules! Depcrateimpl_53 {
() => {
// Module: crate
// Provides: {"impl_53"}
// Dependencies: {}
impl < T : Send + fmt :: Debug > fmt :: Debug for ThreadLocal < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "ThreadLocal {{ local_data: {:?} }}" , self . get ()) } }
};
}
