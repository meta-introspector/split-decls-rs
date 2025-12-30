// Generated macro for TextLen (trait)
macro_rules! Depcrate_traitsTextLen {
() => {
// Module: crate::traits
// Provides: {"TextLen"}
// Dependencies: {}
# [doc = " Primitives with a textual length that can be passed to [`TextSize::of`]."] pub trait TextLen : Copy + Sealed { # [doc = " The textual length of this primitive."] fn text_len (self) -> TextSize ; }
};
}
