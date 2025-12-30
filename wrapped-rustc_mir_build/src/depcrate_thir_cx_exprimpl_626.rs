// Generated macro for impl_626 (impl)
macro_rules! Depcrate_thir_cx_exprimpl_626 {
() => {
// Module: crate::thir::cx::expr
// Provides: {"impl_626"}
// Dependencies: {}
impl ToBorrowKind for hir :: Mutability { fn to_borrow_kind (& self) -> BorrowKind { match * self { hir :: Mutability :: Mut => BorrowKind :: Mut { kind : mir :: MutBorrowKind :: Default } , hir :: Mutability :: Not => BorrowKind :: Shared , } } }
};
}
