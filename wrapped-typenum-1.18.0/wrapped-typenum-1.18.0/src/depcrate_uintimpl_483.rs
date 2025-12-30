// Generated macro for impl_483 (impl)
macro_rules! Depcrate_uintimpl_483 {
() => {
// Module: crate::uint
// Provides: {"impl_483"}
// Dependencies: {}
impl < Un , Bn , Ui , Bi , B > PrivateSetBit < UInt < Ui , Bi > , B > for UInt < Un , Bn > where UInt < Ui , Bi > : Sub < B1 > , Un : PrivateSetBit < Sub1 < UInt < Ui , Bi > > , B > , { type Output = UInt < PrivateSetBitOut < Un , Sub1 < UInt < Ui , Bi > > , B > , Bn > ; # [inline] fn private_set_bit (self , i : UInt < Ui , Bi > , b : B) -> Self :: Output { UInt { msb : self . msb . private_set_bit (i - B1 , b) , lsb : self . lsb , } } }
};
}
