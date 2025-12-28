macro_rules! deps {
    () => {
        PrivateDiv!();
        UInt!();
        Len!();
        Bit!();
        Sub1!();
        B1!();
        UTerm!();
        Length!();
        Unsigned!();
        PrivateDivRem!();
    };
}

macro_rules! impl_473 {
    () => {
        deps!();
        impl < Ul : Unsigned , Bl : Bit , Ur : Unsigned , Br : Bit > Rem < UInt < Ur , Br > > for UInt < Ul , Bl > where UInt < Ul , Bl > : Len , Length < UInt < Ul , Bl > > : Sub < B1 > , () : PrivateDiv < UInt < Ul , Bl > , UInt < Ur , Br > , U0 , U0 , Sub1 < Length < UInt < Ul , Bl > > > > , { type Output = PrivateDivRem < UInt < Ul , Bl > , UInt < Ur , Br > , U0 , U0 , Sub1 < Length < UInt < Ul , Bl > > > > ; # [inline] fn rem (self , rhs : UInt < Ur , Br >) -> Self :: Output { # [allow (clippy :: suspicious_arithmetic_impl)] () . private_div_remainder (self , rhs , UTerm , UTerm , self . len () - B1) } }
    };
}

impl_473!()