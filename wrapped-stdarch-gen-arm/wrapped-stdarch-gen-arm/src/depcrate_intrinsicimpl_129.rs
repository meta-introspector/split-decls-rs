// Generated macro for impl_129 (impl)
macro_rules! Depcrate_intrinsicimpl_129 {
() => {
// Module: crate::intrinsic
// Provides: {"impl_129"}
// Dependencies: {}
impl SubstitutionType { pub fn get (& mut self , ctx : & LocalContext) -> context :: Result < WildString > { match self { Self :: MatchSize (smws) => { smws . perform_match (ctx) ? ; Ok (smws . as_ref () . clone ()) } Self :: MatchKind (kmws) => { kmws . perform_match (ctx) ? ; Ok (kmws . as_ref () . clone ()) } } } }
};
}
