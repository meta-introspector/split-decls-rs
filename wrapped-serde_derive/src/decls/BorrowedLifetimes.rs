macro_rules! BorrowedLifetimes {
    () => {
        enum BorrowedLifetimes { Borrowed (BTreeSet < syn :: Lifetime >) , Static , }
    };
}

BorrowedLifetimes!()