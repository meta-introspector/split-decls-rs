// Generated macro for impl_219 (impl)
macro_rules! Depcrate_fieldimpl_219 {
() => {
// Module: crate::field
// Provides: {"impl_219"}
// Dependencies: {}
impl fmt :: Display for ValueSet < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . values . iter () . fold (& mut f . debug_map () , | dbg , (key , v) | { if let Some (val) = v { val . record (key , dbg) ; } dbg }) . finish () } }
};
}
