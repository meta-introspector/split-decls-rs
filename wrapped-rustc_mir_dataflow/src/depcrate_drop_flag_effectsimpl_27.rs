// Generated macro for impl_27 (impl)
macro_rules! Depcrate_drop_flag_effectsimpl_27 {
() => {
// Module: crate::drop_flag_effects
// Provides: {"impl_27"}
// Dependencies: {}
impl InactiveVariants { fn contains (& self , variant_idx : VariantIdx) -> bool { match self { InactiveVariants :: Inactives (inactives) => inactives . contains (& variant_idx) , InactiveVariants :: Active (active) => variant_idx != * active , } } }
};
}
