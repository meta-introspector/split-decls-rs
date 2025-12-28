macro_rules! ToBorrowKind {
    () => {
        trait ToBorrowKind { fn to_borrow_kind (& self) -> BorrowKind ; }
    };
}

ToBorrowKind!();