// Generated macro for impl_327 (impl)
macro_rules! Depcrate_internedimpl_327 {
() => {
// Module: crate::interned
// Provides: {"impl_327"}
// Dependencies: {}
impl < C > std :: fmt :: Debug for IngredientImpl < C > where C : Configuration , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct (std :: any :: type_name :: < Self > ()) . field ("index" , & self . ingredient_index) . finish () } }
};
}
