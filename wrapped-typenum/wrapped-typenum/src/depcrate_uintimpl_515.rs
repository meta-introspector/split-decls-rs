// Generated macro for impl_515 (impl)
macro_rules! Depcrate_uintimpl_515 {
() => {
// Module: crate::uint
// Provides: {"impl_515"}
// Dependencies: {}
impl < N , D , Q , R , Ui , Bi > PrivateDivIf < N , D , Q , R , UInt < Ui , Bi > , Equal > for () where UInt < Ui , Bi > : Copy + Sub < B1 > , Q : SetBit < UInt < Ui , Bi > , B1 > , () : PrivateDiv < N , D , SetBitOut < Q , UInt < Ui , Bi > , B1 > , U0 , Sub1 < UInt < Ui , Bi > > > , { type Quotient = PrivateDivQuot < N , D , SetBitOut < Q , UInt < Ui , Bi > , B1 > , U0 , Sub1 < UInt < Ui , Bi > > > ; type Remainder = PrivateDivRem < N , D , SetBitOut < Q , UInt < Ui , Bi > , B1 > , U0 , Sub1 < UInt < Ui , Bi > > > ; # [inline] fn private_div_if_quotient (self , n : N , d : D , q : Q , _ : R , i : UInt < Ui , Bi > , _ : Equal ,) -> Self :: Quotient { () . private_div_quotient (n , d , q . set_bit :: < Internal > (i , B1) , U0 :: new () , i - B1) } # [inline] fn private_div_if_remainder (self , n : N , d : D , q : Q , _ : R , i : UInt < Ui , Bi > , _ : Equal ,) -> Self :: Remainder { () . private_div_remainder (n , d , q . set_bit :: < Internal > (i , B1) , U0 :: new () , i - B1) } }
};
}
