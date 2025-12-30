// Generated macro for impl_266 (impl)
macro_rules! Depcrate_privateimpl_266 {
() => {
// Module: crate::private
// Provides: {"impl_266"}
// Dependencies: {}
impl < U : Unsigned , IU : InvertedUnsigned , B : Bit > PrivateInvert < U > for InvertedUInt < IU , B > where IU : PrivateInvert < UInt < U , B > > , { type Output = < IU as PrivateInvert < UInt < U , B > > > :: Output ; # [inline] fn private_invert (self , rhs : U) -> Self :: Output { self . msb . private_invert (UInt { msb : rhs , lsb : self . lsb , }) } }
};
}
