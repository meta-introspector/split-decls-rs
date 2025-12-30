// Generated macro for ref_pat_borrow_kind (function)
macro_rules! Depcrate_builder_matches_utilref_pat_borrow_kind {
() => {
// Module: crate::builder::matches::util
// Provides: {"ref_pat_borrow_kind"}
// Dependencies: {}
# [must_use] pub (crate) fn ref_pat_borrow_kind (ref_mutability : Mutability) -> BorrowKind { match ref_mutability { Mutability :: Mut => BorrowKind :: Mut { kind : MutBorrowKind :: Default } , Mutability :: Not => BorrowKind :: Shared , } }
};
}
