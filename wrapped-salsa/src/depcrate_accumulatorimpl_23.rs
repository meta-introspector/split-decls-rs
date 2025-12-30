// Generated macro for impl_23 (impl)
macro_rules! Depcrate_accumulatorimpl_23 {
() => {
// Module: crate::accumulator
// Provides: {"impl_23"}
// Dependencies: {}
impl < A : Accumulator > IngredientImpl < A > { # [doc = " Find the accumulator ingredient for `A` in the database, if any."] pub fn from_zalsa (zalsa : & Zalsa) -> Option < & Self > { let index = zalsa . lookup_jar_by_type :: < JarImpl < A > > () ; let ingredient = zalsa . lookup_ingredient (index) . assert_type :: < Self > () ; Some (ingredient) } pub fn new (index : IngredientIndex) -> Self { Self { index , phantom : PhantomData , } } pub fn push (& self , zalsa_local : & ZalsaLocal , value : A) { if let Err (()) = zalsa_local . accumulate (self . index , value) { panic ! ("cannot accumulate values outside of an active tracked function") ; } } pub fn index (& self) -> IngredientIndex { self . index } }
};
}
