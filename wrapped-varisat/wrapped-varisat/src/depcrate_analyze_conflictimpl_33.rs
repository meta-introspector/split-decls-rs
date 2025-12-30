// Generated macro for impl_33 (impl)
macro_rules! Depcrate_analyze_conflictimpl_33 {
() => {
// Module: crate::analyze_conflict
// Provides: {"impl_33"}
// Dependencies: {}
impl LevelAbstraction { # [doc = " Add a level to the Bloom filter."] pub fn add (& mut self , level : usize) { self . bits |= 1 << (level % 64) } # [doc = " Test whether a level could be in the Bloom filter."] pub fn test (& self , level : usize) -> bool { self . bits & (1 << (level % 64)) != 0 } }
};
}
