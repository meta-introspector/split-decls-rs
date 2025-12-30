// Generated macro for impl_17 (impl)
macro_rules! Depcrate_rangeimpl_17 {
() => {
// Module: crate::range
// Provides: {"impl_17"}
// Dependencies: {}
impl Sub < TextSize > for TextRange { type Output = TextRange ; # [inline] fn sub (self , offset : TextSize) -> TextRange { self . checked_sub (offset) . expect ("TextRange -offset overflowed") } }
};
}
