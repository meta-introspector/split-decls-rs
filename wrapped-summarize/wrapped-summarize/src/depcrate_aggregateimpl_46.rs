// Generated macro for impl_46 (impl)
macro_rules! Depcrate_aggregateimpl_46 {
() => {
// Module: crate::aggregate
// Provides: {"impl_46"}
// Dependencies: {}
impl < S : Clone > ExtremaSources < S > { pub fn count (& self) -> usize { match * self { ExtremaSources :: Empty => 0 , ExtremaSources :: One (_) => 1 , ExtremaSources :: Count (count) => count , } } pub fn add (& mut self , source : & S) { * self = match self { ExtremaSources :: Empty => ExtremaSources :: One (source . clone ()) , _ => ExtremaSources :: Count (self . count () + 1) , } ; } }
};
}
