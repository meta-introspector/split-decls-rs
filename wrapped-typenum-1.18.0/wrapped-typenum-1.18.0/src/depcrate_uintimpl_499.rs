// Generated macro for impl_499 (impl)
macro_rules! Depcrate_uintimpl_499 {
() => {
// Module: crate::uint
// Provides: {"impl_499"}
// Dependencies: {}
impl < N , D , Q , R , Ui , Bi > PrivateDivIf < N , D , Q , R , UInt < Ui , Bi > , Less > for () where UInt < Ui , Bi > : Sub < B1 > , () : PrivateDiv < N , D , Q , R , Sub1 < UInt < Ui , Bi > > > , { type Quotient = PrivateDivQuot < N , D , Q , R , Sub1 < UInt < Ui , Bi > > > ; type Remainder = PrivateDivRem < N , D , Q , R , Sub1 < UInt < Ui , Bi > > > ; # [inline] fn private_div_if_quotient (self , n : N , d : D , q : Q , r : R , i : UInt < Ui , Bi > , _ : Less ,) -> Self :: Quotient { () . private_div_quotient (n , d , q , r , i - B1) } # [inline] fn private_div_if_remainder (self , n : N , d : D , q : Q , r : R , i : UInt < Ui , Bi > , _ : Less ,) -> Self :: Remainder { () . private_div_remainder (n , d , q , r , i - B1) } }
};
}
