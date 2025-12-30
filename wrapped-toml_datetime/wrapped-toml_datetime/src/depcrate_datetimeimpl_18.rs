// Generated macro for impl_18 (impl)
macro_rules! Depcrate_datetimeimpl_18 {
() => {
// Module: crate::datetime
// Provides: {"impl_18"}
// Dependencies: {}
impl fmt :: Display for Date { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{:04}-{:02}-{:02}" , self . year , self . month , self . day) } }
};
}
