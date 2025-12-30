// Generated macro for trait_ref_is_local_or_fundamental (function)
macro_rules! Depcrate_coherencetrait_ref_is_local_or_fundamental {
() => {
// Module: crate::coherence
// Provides: {"trait_ref_is_local_or_fundamental"}
// Dependencies: {}
pub fn trait_ref_is_local_or_fundamental < I : Interner > (tcx : I , trait_ref : ty :: TraitRef < I >) -> bool { trait_ref . def_id . is_local () || tcx . trait_is_fundamental (trait_ref . def_id) }
};
}
