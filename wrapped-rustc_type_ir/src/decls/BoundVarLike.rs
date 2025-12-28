macro_rules! deps {
    () => {
        Interner!();
    };
}

macro_rules! BoundVarLike {
    () => {
        deps!();
        pub trait BoundVarLike < I : Interner > : Copy + Debug + Hash + Eq { fn var (self) -> ty :: BoundVar ; fn assert_eq (self , var : I :: BoundVarKind) ; }
    };
}

BoundVarLike!()