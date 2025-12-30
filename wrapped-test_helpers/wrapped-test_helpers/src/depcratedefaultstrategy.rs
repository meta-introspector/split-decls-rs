// Generated macro for DefaultStrategy (trait)
macro_rules! DepcrateDefaultStrategy {
() => {
// Module: crate
// Provides: {"DefaultStrategy"}
// Dependencies: {}
# [doc = " Specifies the default strategy for testing a type."] # [doc = ""] # [doc = " This strategy should be what \"makes sense\" to test."] pub trait DefaultStrategy { type Strategy : proptest :: strategy :: Strategy < Value = Self > ; fn default_strategy () -> Self :: Strategy ; }
};
}
