// Generated macro for impl_283 (impl)
macro_rules! Depcrate_spanimpl_283 {
() => {
// Module: crate::span
// Provides: {"impl_283"}
// Dependencies: {}
impl From < Current > for Option < Id > { fn from (cur : Current) -> Self { match cur . inner { CurrentInner :: Current { id , .. } => Some (id) , _ => None , } } }
};
}
