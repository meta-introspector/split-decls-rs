// Generated macro for impl_118 (impl)
macro_rules! Depcrateimpl_118 {
() => {
// Module: crate
// Provides: {"impl_118"}
// Dependencies: {}
impl < K : PartialEq + Hash + Eq , V : Copy + Debug + PartialEq + IndexedVal > Index < V > for IndexMap < K , V > { type Output = K ; fn index (& self , index : V) -> & Self :: Output { let (k , v) = self . index_map . get_index (index . to_index ()) . unwrap () ; assert_eq ! (* v , index , "Provided value doesn't match with indexed value") ; k } }
};
}
