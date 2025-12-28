macro_rules! deps {
    () => {
        Unsigned!();
        PartialDiv!();
        Bit!();
        UInt!();
        Quot!();
    };
}

macro_rules! impl_483 {
    () => {
        deps!();
        impl < Ul : Unsigned , Bl : Bit , Ur : Unsigned , Br : Bit > PartialDiv < UInt < Ur , Br > > for UInt < Ul , Bl > where UInt < Ul , Bl > : Div < UInt < Ur , Br > > + Rem < UInt < Ur , Br > , Output = U0 > , { type Output = Quot < UInt < Ul , Bl > , UInt < Ur , Br > > ; # [inline] fn partial_div (self , rhs : UInt < Ur , Br >) -> Self :: Output { self / rhs } }
    };
}

impl_483!();