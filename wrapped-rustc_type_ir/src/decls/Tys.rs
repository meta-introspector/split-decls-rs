macro_rules! deps {
    () => {
        Interner!();
        TypeFoldable!();
        SliceLike!();
        Ty!();
    };
}

macro_rules! Tys {
    () => {
        deps!();
        pub trait Tys < I : Interner < Tys = Self > > : Copy + Debug + Hash + Eq + SliceLike < Item = I :: Ty > + TypeFoldable < I > + Default { fn inputs (self) -> I :: FnInputTys ; fn output (self) -> I :: Ty ; }
    };
}

Tys!();