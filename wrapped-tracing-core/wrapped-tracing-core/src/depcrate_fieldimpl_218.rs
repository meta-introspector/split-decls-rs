// Generated macro for impl_218 (impl)
macro_rules! Depcrate_fieldimpl_218 {
() => {
// Module: crate::field
// Provides: {"impl_218"}
// Dependencies: {}
impl fmt :: Debug for ValueSet < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . values . iter () . fold (& mut f . debug_struct ("ValueSet") , | dbg , (key , v) | { if let Some (val) = v { val . record (key , dbg) ; } dbg }) . field ("callsite" , & self . callsite ()) . finish () } }
};
}
