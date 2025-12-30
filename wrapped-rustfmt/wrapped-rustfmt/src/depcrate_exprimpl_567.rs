// Generated macro for impl_567 (impl)
macro_rules! Depcrate_exprimpl_567 {
() => {
// Module: crate::expr
// Provides: {"impl_567"}
// Dependencies: {}
impl FloatSymbolParts < '_ > { fn is_fractional_part_zero (& self) -> bool { let zero_literal_regex = static_regex ! (r"^[0_]+$") ; self . fractional_part . is_none_or (| s | zero_literal_regex . is_match (s)) } }
};
}
