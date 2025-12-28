macro_rules! deps {
    () => {
        Interner!();
        TypeFoldable!();
    };
}

macro_rules! DefId {
    () => {
        deps!();
        pub trait DefId < I : Interner > : Copy + Debug + Hash + Eq + TypeFoldable < I > { fn is_local (self) -> bool ; fn as_local (self) -> Option < I :: LocalDefId > ; }
    };
}

DefId!()