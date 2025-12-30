// Generated macro for impl_11 (impl)
macro_rules! Depcrate_cachedimpl_11 {
() => {
// Module: crate::cached
// Provides: {"impl_11"}
// Dependencies: {}
impl < T : Send + fmt :: Debug > fmt :: Debug for CachedThreadLocal < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "ThreadLocal {{ local_data: {:?} }}" , self . get ()) } }
};
}
