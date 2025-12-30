// Generated macro for impl_250 (impl)
macro_rules! Depcrate_privateimpl_250 {
() => {
// Module: crate::private
// Provides: {"impl_250"}
// Dependencies: {}
impl < U : Unsigned , B : Bit > Invert for UInt < U , B > where U : PrivateInvert < InvertedUInt < InvertedUTerm , B > > , { type Output = PrivateInvertOut < U , InvertedUInt < InvertedUTerm , B > > ; # [inline] fn invert (self) -> Self :: Output { self . msb . private_invert (InvertedUInt { msb : InvertedUTerm , lsb : self . lsb , }) } }
};
}
