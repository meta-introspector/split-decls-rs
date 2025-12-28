macro_rules! deps {
    () => {
        GenericArgs!();
        Relate!();
        Interner!();
    };
}

macro_rules! ExprConst {
    () => {
        deps!();
        pub trait ExprConst < I : Interner < ExprConst = Self > > : Copy + Debug + Hash + Eq + Relate < I > { fn args (self) -> I :: GenericArgs ; }
    };
}

ExprConst!()