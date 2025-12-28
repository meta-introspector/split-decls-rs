macro_rules! deps {
    () => {
        PInt!();
        Internal!();
        Cmp!();
        NonZero!();
        InternalMarker!();
        Unsigned!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        # [doc = " X <==> Y"] impl < Pl : Cmp < Pr > + Unsigned + NonZero , Pr : Unsigned + NonZero > Cmp < PInt < Pr > > for PInt < Pl > { type Output = < Pl as Cmp < Pr > > :: Output ; # [inline] fn compare < IM : InternalMarker > (& self , rhs : & PInt < Pr >) -> Self :: Output { self . n . compare :: < Internal > (& rhs . n) } }
    };
}

impl_99!()