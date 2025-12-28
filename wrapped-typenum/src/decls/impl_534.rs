macro_rules! deps {
    () => {
        Len!();
        TArr!();
        B1!();
        Sum!();
        Unsigned!();
        Add1!();
        Length!();
        TypeArray!();
    };
}

macro_rules! impl_534 {
    () => {
        deps!();
        # [doc = " Size of a `TypeArray`"] impl < V , A > Len for TArr < V , A > where A : Len , Length < A > : Add < B1 > , Sum < Length < A > , B1 > : Unsigned , { type Output = Add1 < Length < A > > ; # [inline] fn len (& self) -> Self :: Output { self . rest . len () + B1 } }
    };
}

impl_534!()