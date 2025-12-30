// Generated macro for impl_272 (impl)
macro_rules! Depcrate_inputimpl_272 {
() => {
// Module: crate::input
// Provides: {"impl_272"}
// Dependencies: {}
impl < C : Configuration > Jar for JarImpl < C > { fn create_ingredients (_zalsa : & mut Zalsa , struct_index : crate :: zalsa :: IngredientIndex ,) -> Vec < Box < dyn Ingredient > > { let struct_ingredient : IngredientImpl < C > = IngredientImpl :: new (struct_index) ; std :: iter :: once (Box :: new (struct_ingredient) as _) . chain ((0 .. C :: FIELD_DEBUG_NAMES . len ()) . map (| field_index | { Box :: new (< FieldIngredientImpl < C > > :: new (struct_index , field_index)) as _ })) . collect () } fn id_struct_type_id () -> TypeId { TypeId :: of :: < C :: Struct > () } }
};
}
