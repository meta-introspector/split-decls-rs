// Generated macro for ErasedJar (struct)
macro_rules! Depcrate_zalsaErasedJar {
() => {
// Module: crate::zalsa
// Provides: {"ErasedJar"}
// Dependencies: {}
# [doc = " A type-erased `Jar`, used for ingredient registration."] # [derive (Clone , Copy)] pub struct ErasedJar { kind : JarKind , type_id : fn () -> TypeId , type_name : fn () -> & 'static str , id_struct_type_id : fn () -> TypeId , create_ingredients : fn (& mut Zalsa , IngredientIndex) -> Vec < Box < dyn Ingredient > > , }
};
}
