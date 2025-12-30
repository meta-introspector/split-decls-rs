// Generated macro for impl_740 (impl)
macro_rules! Depcrate_decimal_decimal_patternimpl_740 {
() => {
// Module: crate::decimal::decimal_pattern
// Provides: {"impl_740"}
// Dependencies: {}
impl DecimalSubPattern { # [cfg (feature = "experimental")] pub (crate) fn to_pattern_items (& self) -> Vec < PatternItemCow < '_ , DoublePlaceholderKey > > { use std :: borrow :: Cow ; vec ! [PatternItemCow :: Literal (Cow :: Borrowed (& self . prefix)) , PatternItemCow :: Placeholder (DoublePlaceholderKey :: Place0) , PatternItemCow :: Literal (Cow :: Borrowed (& self . suffix)) ,] } }
};
}
