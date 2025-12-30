// Generated macro for is_generic_bounds_in_order (function)
macro_rules! Depcrate_typesis_generic_bounds_in_order {
() => {
// Module: crate::types
// Provides: {"is_generic_bounds_in_order"}
// Dependencies: {}
fn is_generic_bounds_in_order (generic_bounds : & [ast :: GenericBound]) -> bool { let is_trait = | b : & ast :: GenericBound | match b { ast :: GenericBound :: Outlives (..) => false , ast :: GenericBound :: Trait (..) | ast :: GenericBound :: Use (..) => true , } ; let is_lifetime = | b : & ast :: GenericBound | ! is_trait (b) ; let last_trait_index = generic_bounds . iter () . rposition (is_trait) ; let first_lifetime_index = generic_bounds . iter () . position (is_lifetime) ; match (last_trait_index , first_lifetime_index) { (Some (last_trait_index) , Some (first_lifetime_index)) => { last_trait_index < first_lifetime_index } _ => true , } }
};
}
