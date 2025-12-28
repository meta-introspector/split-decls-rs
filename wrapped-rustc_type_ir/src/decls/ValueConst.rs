macro_rules! deps {
    () => {
        Interner!();
        Ty!();
    };
}

macro_rules! ValueConst {
    () => {
        deps!();
        pub trait ValueConst < I : Interner < ValueConst = Self > > : Copy + Debug + Hash + Eq { fn ty (self) -> I :: Ty ; fn valtree (self) -> I :: ValTree ; }
    };
}

ValueConst!()