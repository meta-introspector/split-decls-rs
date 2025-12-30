// Generated macro for impl_502 (impl)
macro_rules! Depcrate_uintimpl_502 {
() => {
// Module: crate::uint
// Provides: {"impl_502"}
// Dependencies: {}
impl < N , D , Q , R , Ui , Bi > PrivateDivIf < N , D , Q , R , UInt < Ui , Bi > , Greater > for () where D : Copy , UInt < Ui , Bi > : Copy + Sub < B1 > , R : Sub < D > , Q : SetBit < UInt < Ui , Bi > , B1 > , () : PrivateDiv < N , D , SetBitOut < Q , UInt < Ui , Bi > , B1 > , Diff < R , D > , Sub1 < UInt < Ui , Bi > > > , { type Quotient = PrivateDivQuot < N , D , SetBitOut < Q , UInt < Ui , Bi > , B1 > , Diff < R , D > , Sub1 < UInt < Ui , Bi > > > ; type Remainder = PrivateDivRem < N , D , SetBitOut < Q , UInt < Ui , Bi > , B1 > , Diff < R , D > , Sub1 < UInt < Ui , Bi > > > ; # [inline] fn private_div_if_quotient (self , n : N , d : D , q : Q , r : R , i : UInt < Ui , Bi > , _ : Greater ,) -> Self :: Quotient { () . private_div_quotient (n , d , q . set_bit :: < Internal > (i , B1) , r - d , i - B1) } # [inline] fn private_div_if_remainder (self , n : N , d : D , q : Q , r : R , i : UInt < Ui , Bi > , _ : Greater ,) -> Self :: Remainder { () . private_div_remainder (n , d , q . set_bit :: < Internal > (i , B1) , r - d , i - B1) } }
};
}
