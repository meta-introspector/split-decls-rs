macro_rules! deps {
    () => {
        Ty!();
        Interner!();
    };
}

macro_rules! ValueConst {
    () => {
        deps!();
        pub trait ValueConst < I : Interner < ValueConst = Self > > : Copy + Debug + Hash + Eq { fn ty (self) -> I :: Ty ; fn valtree (self) -> I :: ValTree ; }
    };
}

ValueConst!();