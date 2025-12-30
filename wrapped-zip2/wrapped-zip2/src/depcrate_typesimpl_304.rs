// Generated macro for impl_304 (impl)
macro_rules! Depcrate_typesimpl_304 {
() => {
// Module: crate::types
// Provides: {"impl_304"}
// Dependencies: {}
impl fmt :: Display for DateTime { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{:04}-{:02}-{:02} {:02}:{:02}:{:02}" , self . year () , self . month () , self . day () , self . hour () , self . minute () , self . second ()) } }
};
}
