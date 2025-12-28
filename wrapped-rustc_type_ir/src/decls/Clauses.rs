macro_rules! deps {
    () => {
        TypeSuperFoldable!();
        SliceLike!();
        Clause!();
        Interner!();
        TypeSuperVisitable!();
        Flags!();
    };
}

macro_rules! Clauses {
    () => {
        deps!();
        pub trait Clauses < I : Interner < Clauses = Self > > : Copy + Debug + Hash + Eq + TypeSuperVisitable < I > + TypeSuperFoldable < I > + Flags + SliceLike < Item = I :: Clause > { }
    };
}

Clauses!();