// Generated macro for impl_264 (impl)
macro_rules! Depcrate_privateimpl_264 {
() => {
// Module: crate::private
// Provides: {"impl_264"}
// Dependencies: {}
impl < IU : InvertedUnsigned , B : Bit > Invert for InvertedUInt < IU , B > where IU : PrivateInvert < UInt < UTerm , B > > , { type Output = < IU as PrivateInvert < UInt < UTerm , B > > > :: Output ; # [inline] fn invert (self) -> Self :: Output { self . msb . private_invert (UInt { msb : UTerm , lsb : self . lsb , }) } }
};
}
