// Generated macro for impl_44 (impl)
macro_rules! Depcrateimpl_44 {
() => {
// Module: crate
// Provides: {"impl_44"}
// Dependencies: {}
impl From < RangeOrOffset > for TextRange { fn from (selection : RangeOrOffset) -> Self { match selection { RangeOrOffset :: Range (it) => it , RangeOrOffset :: Offset (it) => TextRange :: empty (it) , } } }
};
}
