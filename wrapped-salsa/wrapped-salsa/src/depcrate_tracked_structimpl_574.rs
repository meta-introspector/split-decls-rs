// Generated macro for impl_574 (impl)
macro_rules! Depcrate_tracked_structimpl_574 {
() => {
// Module: crate::tracked_struct
// Provides: {"impl_574"}
// Dependencies: {}
impl < C > std :: fmt :: Debug for IngredientImpl < C > where C : Configuration , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct (std :: any :: type_name :: < Self > ()) . field ("ingredient_index" , & self . ingredient_index) . finish () } }
};
}
