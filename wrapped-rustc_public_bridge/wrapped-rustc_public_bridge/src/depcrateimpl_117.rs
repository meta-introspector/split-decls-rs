// Generated macro for impl_117 (impl)
macro_rules! Depcrateimpl_117 {
() => {
// Module: crate
// Provides: {"impl_117"}
// Dependencies: {}
impl < K : PartialEq + Hash + Eq , V : Copy + Debug + PartialEq + IndexedVal > IndexMap < K , V > { pub fn create_or_fetch (& mut self , key : K) -> V { let len = self . index_map . len () ; let v = self . index_map . entry (key) . or_insert (V :: to_val (len)) ; * v } }
};
}
