macro_rules! deps {
    () => {
        Unsigned!();
        Sum!();
        Add1!();
        B1!();
        Length!();
        TArr!();
        Len!();
        TypeArray!();
    };
}

macro_rules! impl_534 {
    () => {
        deps!();
        # [doc = " Size of a `TypeArray`"] impl < V , A > Len for TArr < V , A > where A : Len , Length < A > : Add < B1 > , Sum < Length < A > , B1 > : Unsigned , { type Output = Add1 < Length < A > > ; # [inline] fn len (& self) -> Self :: Output { self . rest . len () + B1 } }
    };
}

impl_534!();