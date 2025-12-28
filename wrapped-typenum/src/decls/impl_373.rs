macro_rules! deps {
    () => {
        PrivateSub!();
        PrivateSubOut!();
        Trim!();
        Bit!();
        Unsigned!();
        UInt!();
        TrimOut!();
    };
}

macro_rules! impl_373 {
    () => {
        deps!();
        # [doc = " Subtracting unsigned integers. We just do our `PrivateSub` and then `Trim` the output."] impl < Ul : Unsigned , Bl : Bit , Ur : Unsigned > Sub < Ur > for UInt < Ul , Bl > where UInt < Ul , Bl > : PrivateSub < Ur > , PrivateSubOut < UInt < Ul , Bl > , Ur > : Trim , { type Output = TrimOut < PrivateSubOut < UInt < Ul , Bl > , Ur > > ; # [inline] fn sub (self , rhs : Ur) -> Self :: Output { self . private_sub (rhs) . trim () } }
    };
}

impl_373!()