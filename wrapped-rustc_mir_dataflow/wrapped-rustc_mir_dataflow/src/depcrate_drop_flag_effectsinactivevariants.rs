// Generated macro for InactiveVariants (enum)
macro_rules! Depcrate_drop_flag_effectsInactiveVariants {
() => {
// Module: crate::drop_flag_effects
// Provides: {"InactiveVariants"}
// Dependencies: {}
# [doc = " Indicates which variants are inactive at a `SwitchInt` edge by listing their `VariantIdx`s or"] # [doc = " specifying the single active variant's `VariantIdx`."] pub (crate) enum InactiveVariants { Inactives (SmallVec < [VariantIdx ; 4] >) , Active (VariantIdx) , }
};
}
