macro_rules! UnionIter {
    () => {
        pub (crate) struct UnionIter < X : Iterator , Y : Iterator > { xs : Peekable < X > , ys : Peekable < Y > , }
    };
}

UnionIter!()