// Generated macro for impl_73 (impl)
macro_rules! Depcrate_tagimpl_73 {
() => {
// Module: crate::tag
// Provides: {"impl_73"}
// Dependencies: {}
impl fmt :: Display for Tag { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { if let Some (prefix) = self . prefix { write ! (f , "{}.{}" , prefix , self . suffix) } else { self . suffix . fmt (f) } } }
};
}
