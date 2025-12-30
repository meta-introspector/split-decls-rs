// Generated macro for impl_23 (impl)
macro_rules! Depcrate_stackimpl_23 {
() => {
// Module: crate::stack
// Provides: {"impl_23"}
// Dependencies: {}
impl < Inner , Outer > fmt :: Debug for Stack < Inner , Outer > where Inner : fmt :: Debug , Outer : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if f . alternate () { write ! (f , "{:#?},\n{:#?}" , self . outer , self . inner) } else { write ! (f , "{:?}, {:?}" , self . outer , self . inner) } } }
};
}
