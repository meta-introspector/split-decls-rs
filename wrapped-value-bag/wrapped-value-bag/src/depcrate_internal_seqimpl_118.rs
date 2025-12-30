// Generated macro for impl_118 (impl)
macro_rules! Depcrate_internal_seqimpl_118 {
() => {
// Module: crate::internal::seq
// Provides: {"impl_118"}
// Dependencies: {}
impl < 'a , S : Seq + ? Sized > Seq for & 'a S { fn visit (& self , visitor : & mut dyn Visitor < '_ >) { (* * self) . visit (visitor) } fn borrowed_visit < 'v > (& 'v self , visitor : & mut dyn Visitor < 'v >) { (* * self) . borrowed_visit (visitor) } }
};
}
