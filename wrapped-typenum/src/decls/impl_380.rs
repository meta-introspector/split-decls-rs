macro_rules! deps {
    () => {
        Bit!();
        Unsigned!();
        UInt!();
        PrivateAnd!();
        Trim!();
        PrivateAndOut!();
        TrimOut!();
    };
}

macro_rules! impl_380 {
    () => {
        deps!();
        # [doc = " Anding unsigned integers."] # [doc = " We use our `PrivateAnd` operator and then `Trim` the output."] impl < Ul : Unsigned , Bl : Bit , Ur : Unsigned > BitAnd < Ur > for UInt < Ul , Bl > where UInt < Ul , Bl > : PrivateAnd < Ur > , PrivateAndOut < UInt < Ul , Bl > , Ur > : Trim , { type Output = TrimOut < PrivateAndOut < UInt < Ul , Bl > , Ur > > ; # [inline] fn bitand (self , rhs : Ur) -> Self :: Output { self . private_and (rhs) . trim () } }
    };
}

impl_380!();