// Generated macro for impl_21 (impl)
macro_rules! Depcrate_accumulatorimpl_21 {
() => {
// Module: crate::accumulator
// Provides: {"impl_21"}
// Dependencies: {}
impl < A : Accumulator > Jar for JarImpl < A > { fn create_ingredients (_zalsa : & mut Zalsa , first_index : IngredientIndex ,) -> Vec < Box < dyn Ingredient > > { vec ! [Box :: new (< IngredientImpl < A >>:: new (first_index))] } fn id_struct_type_id () -> TypeId { TypeId :: of :: < A > () } }
};
}
