// Generated macro for impl_213 (impl)
macro_rules! Depcrate_solve_inspect_buildimpl_213 {
() => {
// Module: crate::solve::inspect::build
// Provides: {"impl_213"}
// Dependencies: {}
impl < I : Interner > WipProbe < I > { fn finalize (self) -> inspect :: Probe < I > { inspect :: Probe { steps : self . steps . into_iter () . map (WipProbeStep :: finalize) . collect () , kind : self . kind . unwrap () , final_state : self . final_state . unwrap () , } } }
};
}
