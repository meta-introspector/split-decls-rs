// Generated macro for impl_18 (impl)
macro_rules! Depcrate_enumerableimpl_18 {
() => {
// Module: crate::enumerable
// Provides: {"impl_18"}
// Dependencies: {}
impl < T , E > Enumerable for Result < T , E > where T : Valuable , E : Valuable , { fn definition (& self) -> EnumDef < '_ > { EnumDef :: new_static ("Result" , RESULT_VARIANTS) } fn variant (& self) -> Variant < '_ > { match self { Ok (_) => Variant :: Static (& RESULT_VARIANTS [0]) , Err (_) => Variant :: Static (& RESULT_VARIANTS [1]) , } } }
};
}
