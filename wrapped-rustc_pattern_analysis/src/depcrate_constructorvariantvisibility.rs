// Generated macro for VariantVisibility (enum)
macro_rules! Depcrate_constructorVariantVisibility {
() => {
// Module: crate::constructor
// Provides: {"VariantVisibility"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] pub enum VariantVisibility { # [doc = " Variant that doesn't fit the other cases, i.e. most variants."] Visible , # [doc = " Variant behind an unstable gate or with the `#[doc(hidden)]` attribute. It will not be"] # [doc = " mentioned in diagnostics unless the user mentioned it first."] Hidden , # [doc = " Variant that matches no value. E.g. `Some::<Option<!>>` if the `exhaustive_patterns` feature"] # [doc = " is enabled. Like `Hidden`, it will not be mentioned in diagnostics unless the user mentioned"] # [doc = " it first."] Empty , }
};
}
