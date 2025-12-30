// Generated macro for Token (trait)
macro_rules! Depcrate_astToken {
() => {
// Module: crate::ast
// Provides: {"Token"}
// Dependencies: {}
# [doc = " Trait for types that represent single tokens, such as keywords"] # [doc = " and operators."] pub (crate) trait Token where Self : Copy + Sized , { fn token (& self) -> & 'static str ; }
};
}
