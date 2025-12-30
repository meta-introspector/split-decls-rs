// Generated macro for BottPeriodicityCache (struct)
macro_rules! Depcrate_bott_periodicityBottPeriodicityCache {
() => {
// Module: crate::bott_periodicity
// Provides: {"BottPeriodicityCache"}
// Dependencies: {}
# [doc = " Cache for Bott periodicity computations"] # [derive (Debug , Clone)] pub struct BottPeriodicityCache { cache : std :: collections :: HashMap < usize , AbstractionBundle > , pub current_generation : usize , pub levels : Vec < AbstractionBundle > , pub fiber_bundles : Vec < String > , }
};
}
