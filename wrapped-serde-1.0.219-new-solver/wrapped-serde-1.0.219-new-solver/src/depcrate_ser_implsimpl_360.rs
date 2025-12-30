// Generated macro for impl_360 (impl)
macro_rules! Depcrate_ser_implsimpl_360 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_360"}
// Dependencies: {}
impl < T > Serialize for PhantomData < T > where T : ? Sized , { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_unit_struct ("PhantomData") } }
};
}
