// Generated macro for impl_37 (impl)
macro_rules! Depcrate_dateimpl_37 {
() => {
// Module: crate::date
// Provides: {"impl_37"}
// Dependencies: {}
impl Display for Date { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { write ! (formatter , "{:04}-{:02}-{:02}" , self . year , self . month , self . day ,) } }
};
}
