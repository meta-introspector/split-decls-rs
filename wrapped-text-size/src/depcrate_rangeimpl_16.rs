// Generated macro for impl_16 (impl)
macro_rules! Depcrate_rangeimpl_16 {
() => {
// Module: crate::range
// Provides: {"impl_16"}
// Dependencies: {}
impl Add < TextSize > for TextRange { type Output = TextRange ; # [inline] fn add (self , offset : TextSize) -> TextRange { self . checked_add (offset) . expect ("TextRange +offset overflowed") } }
};
}
