// Generated macro for impl_37 (impl)
macro_rules! Depcrate_attrimpl_37 {
() => {
// Module: crate::attr
// Provides: {"impl_37"}
// Dependencies: {}
impl < 'a > ShortestName for Database < 'a > { fn shortest_name_by_oid (& self , oid : & ObjectIdentifier) -> Option < & 'a str > { let mut best_match : Option < & 'a str > = None ; for m in self . find_names_for_oid (* oid) { if let Some (previous) = best_match { if m . len () < previous . len () { best_match = Some (m) ; } } else { best_match = Some (m) ; } } best_match } }
};
}
