// Generated macro for impl_638 (impl)
macro_rules! Depcrate_ty_kindimpl_638 {
() => {
// Module: crate::ty_kind
// Provides: {"impl_638"}
// Dependencies: {}
impl AliasTyKind { pub fn descr (self) -> & 'static str { match self { AliasTyKind :: Projection => "associated type" , AliasTyKind :: Inherent => "inherent associated type" , AliasTyKind :: Opaque => "opaque type" , AliasTyKind :: Free => "type alias" , } } }
};
}
