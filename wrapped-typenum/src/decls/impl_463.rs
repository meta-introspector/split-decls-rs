macro_rules! deps {
    () => {
        Trim!();
        TrimOut!();
        PrivateSetBitOut!();
        SetBit!();
        PrivateSetBit!();
        InternalMarker!();
    };
}

macro_rules! impl_463 {
    () => {
        deps!();
        impl < N , I , B > SetBit < I , B > for N where N : PrivateSetBit < I , B > , PrivateSetBitOut < N , I , B > : Trim , { type Output = TrimOut < PrivateSetBitOut < N , I , B > > ; # [inline] fn set_bit < IM : InternalMarker > (self , i : I , b : B) -> Self :: Output { self . private_set_bit (i , b) . trim () } }
    };
}

impl_463!();