// Generated macro for impl_252 (impl)
macro_rules! Depcrate_privateimpl_252 {
() => {
// Module: crate::private
// Provides: {"impl_252"}
// Dependencies: {}
impl < IU : InvertedUnsigned , U : Unsigned , B : Bit > PrivateInvert < IU > for UInt < U , B > where U : PrivateInvert < InvertedUInt < IU , B > > , { type Output = PrivateInvertOut < U , InvertedUInt < IU , B > > ; # [inline] fn private_invert (self , rhs : IU) -> Self :: Output { self . msb . private_invert (InvertedUInt { msb : rhs , lsb : self . lsb , }) } }
};
}
