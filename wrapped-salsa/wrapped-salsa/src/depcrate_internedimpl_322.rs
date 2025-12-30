// Generated macro for impl_322 (impl)
macro_rules! Depcrate_internedimpl_322 {
() => {
// Module: crate::interned
// Provides: {"impl_322"}
// Dependencies: {}
impl < C : Configuration > Jar for JarImpl < C > { fn create_ingredients (_zalsa : & mut Zalsa , first_index : IngredientIndex ,) -> Vec < Box < dyn Ingredient > > { vec ! [Box :: new (IngredientImpl ::< C >:: new (first_index)) as _] } fn id_struct_type_id () -> TypeId { TypeId :: of :: < C :: Struct < 'static > > () } }
};
}
