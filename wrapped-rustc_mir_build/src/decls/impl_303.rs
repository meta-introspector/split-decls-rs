macro_rules! deps {
    () => {
        ToBorrowKind!();
    };
}

macro_rules! impl_303 {
    () => {
        deps!();
        impl ToBorrowKind for hir :: Mutability { fn to_borrow_kind (& self) -> BorrowKind { match * self { hir :: Mutability :: Mut => BorrowKind :: Mut { kind : mir :: MutBorrowKind :: Default } , hir :: Mutability :: Not => BorrowKind :: Shared , } } }
    };
}

impl_303!()