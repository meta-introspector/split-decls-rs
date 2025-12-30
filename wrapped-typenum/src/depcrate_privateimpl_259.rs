// Generated macro for impl_259 (impl)
macro_rules! Depcrate_privateimpl_259 {
() => {
// Module: crate::private
// Provides: {"impl_259"}
// Dependencies: {}
impl < U : Unsigned , B : Bit > Invert for UInt < U , B > where U : PrivateInvert < InvertedUInt < InvertedUTerm , B > > , { type Output = PrivateInvertOut < U , InvertedUInt < InvertedUTerm , B > > ; # [inline] fn invert (self) -> Self :: Output { self . msb . private_invert (InvertedUInt { msb : InvertedUTerm , lsb : self . lsb , }) } }
};
}
