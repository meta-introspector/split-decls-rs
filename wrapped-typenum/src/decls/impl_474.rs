macro_rules! deps {
    () => {
        UInt!();
        PrivateDiv!();
        Compare!();
        PrivateDivIfRem!();
        Internal!();
        TrimOut!();
        PrivateDivIf!();
        GetBit!();
        UTerm!();
        GetBitOut!();
        Trim!();
        PrivateDivIfQuot!();
        Cmp!();
    };
}

macro_rules! impl_474 {
    () => {
        deps!();
        impl < N , D , Q , I > PrivateDiv < N , D , Q , U0 , I > for () where N : GetBit < I > , UInt < UTerm , GetBitOut < N , I > > : Trim , TrimOut < UInt < UTerm , GetBitOut < N , I > > > : Cmp < D > , () : PrivateDivIf < N , D , Q , TrimOut < UInt < UTerm , GetBitOut < N , I > > > , I , Compare < TrimOut < UInt < UTerm , GetBitOut < N , I > > > , D > , > , { type Quotient = PrivateDivIfQuot < N , D , Q , TrimOut < UInt < UTerm , GetBitOut < N , I > > > , I , Compare < TrimOut < UInt < UTerm , GetBitOut < N , I > > > , D > , > ; type Remainder = PrivateDivIfRem < N , D , Q , TrimOut < UInt < UTerm , GetBitOut < N , I > > > , I , Compare < TrimOut < UInt < UTerm , GetBitOut < N , I > > > , D > , > ; # [inline] fn private_div_quotient (self , n : N , d : D , q : Q , _ : U0 , i : I) -> Self :: Quotient { let r = (UInt { msb : UTerm , lsb : n . get_bit :: < Internal > (& i) , }) . trim () ; let r_cmp_d = r . compare :: < Internal > (& d) ; () . private_div_if_quotient (n , d , q , r , i , r_cmp_d) } # [inline] fn private_div_remainder (self , n : N , d : D , q : Q , _ : U0 , i : I) -> Self :: Remainder { let r = (UInt { msb : UTerm , lsb : n . get_bit :: < Internal > (& i) , }) . trim () ; let r_cmp_d = r . compare :: < Internal > (& d) ; () . private_div_if_remainder (n , d , q , r , i , r_cmp_d) } }
    };
}

impl_474!();