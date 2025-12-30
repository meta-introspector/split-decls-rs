// Generated macro for IsNormalized (enum)
macro_rules! Depcrate_quick_checkIsNormalized {
() => {
// Module: crate::quick_check
// Provides: {"IsNormalized"}
// Dependencies: {}
# [doc = " QuickCheck quickly determines if a string is normalized, it can return"] # [doc = " `Maybe`"] # [doc = ""] # [doc = " The QuickCheck algorithm can quickly determine if a text is or isn't"] # [doc = " normalized without any allocations in many cases, but it has to be able to"] # [doc = " return `Maybe` when a full decomposition and recomposition is necessary."] # [derive (Debug , Eq , PartialEq)] pub enum IsNormalized { # [doc = " The text is definitely normalized."] Yes , # [doc = " The text is definitely not normalized."] No , # [doc = " The text may be normalized."] Maybe , }
};
}
