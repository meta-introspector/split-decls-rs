macro_rules! BorrowAttribute {
    () => {
        struct BorrowAttribute { path : syn :: Path , lifetimes : Option < BTreeSet < syn :: Lifetime > > , }
    };
}

BorrowAttribute!();