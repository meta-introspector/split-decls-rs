macro_rules! ref_pat_borrow_kind {
    () => {
        # [must_use] pub (crate) fn ref_pat_borrow_kind (ref_mutability : Mutability) -> BorrowKind { match ref_mutability { Mutability :: Mut => BorrowKind :: Mut { kind : MutBorrowKind :: Default } , Mutability :: Not => BorrowKind :: Shared , } }
    };
}

ref_pat_borrow_kind!();