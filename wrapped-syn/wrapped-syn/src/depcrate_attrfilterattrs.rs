// Generated macro for FilterAttrs (trait)
macro_rules! Depcrate_attrFilterAttrs {
() => {
// Module: crate::attr
// Provides: {"FilterAttrs"}
// Dependencies: {}
pub trait FilterAttrs < 'a > { type Ret : Iterator < Item = & 'a Attribute > ; fn outer (self) -> Self :: Ret ; fn inner (self) -> Self :: Ret ; }
};
}
