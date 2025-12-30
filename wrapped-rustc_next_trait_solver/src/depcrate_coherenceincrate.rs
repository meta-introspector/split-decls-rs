// Generated macro for InCrate (enum)
macro_rules! Depcrate_coherenceInCrate {
() => {
// Module: crate::coherence
// Provides: {"InCrate"}
// Dependencies: {}
# [doc = " Whether we do the orphan check relative to this crate or to some remote crate."] # [derive (Copy , Clone , Debug)] pub enum InCrate { Local { mode : OrphanCheckMode } , Remote , }
};
}
