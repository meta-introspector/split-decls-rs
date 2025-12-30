// Generated macro for impl_628 (impl)
macro_rules! Depcrate_ule_nicheimpl_628 {
() => {
// Module: crate::ule::niche
// Provides: {"impl_628"}
// Dependencies: {}
impl < U : NicheBytes < N > + ULE + core :: fmt :: Debug , const N : usize > core :: fmt :: Debug for NichedOptionULE < U , N > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { self . get () . fmt (f) } }
};
}
