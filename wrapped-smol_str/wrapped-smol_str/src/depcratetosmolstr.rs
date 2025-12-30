// Generated macro for ToSmolStr (trait)
macro_rules! DepcrateToSmolStr {
() => {
// Module: crate
// Provides: {"ToSmolStr"}
// Dependencies: {}
# [doc = " Convert value to [`SmolStr`] using [`fmt::Display`], potentially without allocating."] # [doc = ""] # [doc = " Almost identical to [`ToString`], but converts to `SmolStr` instead."] pub trait ToSmolStr { fn to_smolstr (& self) -> SmolStr ; }
};
}
