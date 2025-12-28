macro_rules! deps {
    () => {
        Interner!();
        TypeRelation!();
        RelateResult!();
        TypeFoldable!();
    };
}

macro_rules! Relate {
    () => {
        deps!();
        pub trait Relate < I : Interner > : TypeFoldable < I > + PartialEq + Copy { fn relate < R : TypeRelation < I > > (relation : & mut R , a : Self , b : Self) -> RelateResult < I , Self > ; }
    };
}

Relate!()