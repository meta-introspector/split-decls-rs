// Generated macro for impl_209 (impl)
macro_rules! Depcrate_asmimpl_209 {
() => {
// Module: crate::asm
// Provides: {"impl_209"}
// Dependencies: {}
impl fmt :: Display for InlineAsmRegOrRegClass { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Reg (r) => write ! (f , "\"{}\"" , r . name ()) , Self :: RegClass (r) => write ! (f , "{}" , r . name ()) , } } }
};
}
