// Generated macro for impl_116 (impl)
macro_rules! Depcrate_internal_seqimpl_116 {
() => {
// Module: crate::internal::seq
// Provides: {"impl_116"}
// Dependencies: {}
impl < 'a , I , T > Seq for SeqSlice < 'a , I , T > where I : AsRef < [T] > + ? Sized + 'a , & 'a T : Into < ValueBag < 'a > > , { fn visit (& self , visitor : & mut dyn Visitor < '_ >) { for v in self . as_ref () . iter () { if let ControlFlow :: Break (()) = visitor . element (v . into ()) { return ; } } } fn borrowed_visit < 'v > (& 'v self , visitor : & mut dyn Visitor < 'v >) { for v in self . as_ref () . iter () { if let ControlFlow :: Break (()) = visitor . borrowed_element (v . into ()) { return ; } } } }
};
}
