macro_rules! deps {
    () => {
        Interner!();
        TypeFoldable!();
    };
}

macro_rules! Span {
    () => {
        deps!();
        pub trait Span < I : Interner > : Copy + Debug + Hash + Eq + TypeFoldable < I > { fn dummy () -> Self ; }
    };
}

Span!();