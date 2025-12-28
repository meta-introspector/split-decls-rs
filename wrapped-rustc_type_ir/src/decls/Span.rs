macro_rules! deps {
    () => {
        TypeFoldable!();
        Interner!();
    };
}

macro_rules! Span {
    () => {
        deps!();
        pub trait Span < I : Interner > : Copy + Debug + Hash + Eq + TypeFoldable < I > { fn dummy () -> Self ; }
    };
}

Span!()