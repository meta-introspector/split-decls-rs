// Generated macro for impl_72 (impl)
macro_rules! Depcrate_cursorimpl_72 {
() => {
// Module: crate::cursor
// Provides: {"impl_72"}
// Dependencies: {}
impl < 'a > fmt :: Debug for Cursor < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Cursor") . field ("ptr" , & self . ptr) . field ("scope" , & self . scope) . field ("entry" , self . entry ()) . finish () } }
};
}
