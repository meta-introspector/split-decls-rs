// Generated macro for impl_25 (impl)
macro_rules! Depcrate_accumulatorimpl_25 {
() => {
// Module: crate::accumulator
// Provides: {"impl_25"}
// Dependencies: {}
impl < A > std :: fmt :: Debug for IngredientImpl < A > where A : Accumulator , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct (std :: any :: type_name :: < Self > ()) . field ("index" , & self . index) . finish () } }
};
}
