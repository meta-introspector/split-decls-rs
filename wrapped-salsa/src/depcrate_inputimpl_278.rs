// Generated macro for impl_278 (impl)
macro_rules! Depcrate_inputimpl_278 {
() => {
// Module: crate::input
// Provides: {"impl_278"}
// Dependencies: {}
impl < C : Configuration > std :: fmt :: Debug for IngredientImpl < C > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct (std :: any :: type_name :: < Self > ()) . field ("index" , & self . ingredient_index) . finish () } }
};
}
