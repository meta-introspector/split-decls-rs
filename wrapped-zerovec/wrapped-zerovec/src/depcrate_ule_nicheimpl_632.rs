// Generated macro for impl_632 (impl)
macro_rules! Depcrate_ule_nicheimpl_632 {
() => {
// Module: crate::ule::niche
// Provides: {"impl_632"}
// Dependencies: {}
impl < U : NicheBytes < N > + ULE + PartialEq , const N : usize > PartialEq for NichedOptionULE < U , N > { fn eq (& self , other : & Self) -> bool { self . get () . eq (& other . get ()) } }
};
}
