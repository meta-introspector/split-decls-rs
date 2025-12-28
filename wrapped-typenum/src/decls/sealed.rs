macro_rules! deps {
    () => {
        Unsigned!();
        Equal!();
        B1!();
        PInt!();
        NInt!();
        Greater!();
        Less!();
        TArr!();
        Bit!();
        UTerm!();
        UInt!();
        ATerm!();
        B0!();
        Z0!();
        NonZero!();
    };
}

macro_rules! sealed {
    () => {
        deps!();
        mod sealed { use crate :: { ATerm , Bit , Equal , Greater , Less , NInt , NonZero , PInt , TArr , UInt , UTerm , Unsigned , B0 , B1 , Z0 , } ; pub trait Sealed { } impl Sealed for B0 { } impl Sealed for B1 { } impl Sealed for UTerm { } impl < U : Unsigned , B : Bit > Sealed for UInt < U , B > { } impl Sealed for Z0 { } impl < U : Unsigned + NonZero > Sealed for PInt < U > { } impl < U : Unsigned + NonZero > Sealed for NInt < U > { } impl Sealed for Less { } impl Sealed for Equal { } impl Sealed for Greater { } impl Sealed for ATerm { } impl < V , A > Sealed for TArr < V , A > { } }
    };
}

sealed!();