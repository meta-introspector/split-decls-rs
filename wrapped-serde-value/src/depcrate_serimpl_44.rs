// Generated macro for impl_44 (impl)
macro_rules! Depcrate_serimpl_44 {
() => {
// Module: crate::ser
// Provides: {"impl_44"}
// Dependencies: {}
impl fmt :: Display for SerializerError { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { match * self { SerializerError :: Custom (ref s) => fmt . write_str (s) , } } }
};
}
