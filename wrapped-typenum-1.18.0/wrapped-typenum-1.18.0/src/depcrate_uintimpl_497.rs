// Generated macro for impl_497 (impl)
macro_rules! Depcrate_uintimpl_497 {
() => {
// Module: crate::uint
// Provides: {"impl_497"}
// Dependencies: {}
impl < N , D , Q , Ur , Br , I > PrivateDiv < N , D , Q , UInt < Ur , Br > , I > for () where N : GetBit < I > , UInt < UInt < Ur , Br > , GetBitOut < N , I > > : Cmp < D > , () : PrivateDivIf < N , D , Q , UInt < UInt < Ur , Br > , GetBitOut < N , I > > , I , Compare < UInt < UInt < Ur , Br > , GetBitOut < N , I > > , D > , > , { type Quotient = PrivateDivIfQuot < N , D , Q , UInt < UInt < Ur , Br > , GetBitOut < N , I > > , I , Compare < UInt < UInt < Ur , Br > , GetBitOut < N , I > > , D > , > ; type Remainder = PrivateDivIfRem < N , D , Q , UInt < UInt < Ur , Br > , GetBitOut < N , I > > , I , Compare < UInt < UInt < Ur , Br > , GetBitOut < N , I > > , D > , > ; # [inline] fn private_div_quotient (self , n : N , d : D , q : Q , r : UInt < Ur , Br > , i : I) -> Self :: Quotient { let r = UInt { msb : r , lsb : n . get_bit :: < Internal > (& i) , } ; let r_cmp_d = r . compare :: < Internal > (& d) ; () . private_div_if_quotient (n , d , q , r , i , r_cmp_d) } # [inline] fn private_div_remainder (self , n : N , d : D , q : Q , r : UInt < Ur , Br > , i : I) -> Self :: Remainder { let r = UInt { msb : r , lsb : n . get_bit :: < Internal > (& i) , } ; let r_cmp_d = r . compare :: < Internal > (& d) ; () . private_div_if_remainder (n , d , q , r , i , r_cmp_d) } }
};
}
