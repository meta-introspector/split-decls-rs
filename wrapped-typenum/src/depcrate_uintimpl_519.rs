// Generated macro for impl_519 (impl)
macro_rules! Depcrate_uintimpl_519 {
() => {
// Module: crate::uint
// Provides: {"impl_519"}
// Dependencies: {}
impl < N , D , Q , R > PrivateDivIf < N , D , Q , R , U0 , Equal > for () where Q : SetBit < U0 , B1 > , { type Quotient = SetBitOut < Q , U0 , B1 > ; type Remainder = U0 ; # [inline] fn private_div_if_quotient (self , _ : N , _ : D , q : Q , _ : R , i : U0 , _ : Equal) -> Self :: Quotient { q . set_bit :: < Internal > (i , B1) } # [inline] fn private_div_if_remainder (self , _ : N , _ : D , _ : Q , _ : R , i : U0 , _ : Equal) -> Self :: Remainder { i } }
};
}
