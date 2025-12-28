macro_rules! deps {
    () => {
        GenericArgs!();
        Interner!();
        Relate!();
    };
}

macro_rules! ExprConst {
    () => {
        deps!();
        pub trait ExprConst < I : Interner < ExprConst = Self > > : Copy + Debug + Hash + Eq + Relate < I > { fn args (self) -> I :: GenericArgs ; }
    };
}

ExprConst!();