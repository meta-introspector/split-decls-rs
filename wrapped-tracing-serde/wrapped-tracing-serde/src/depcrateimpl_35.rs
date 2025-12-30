// Generated macro for impl_35 (impl)
macro_rules! Depcrateimpl_35 {
() => {
// Module: crate
// Provides: {"impl_35"}
// Dependencies: {}
impl < S : SerializeStruct > SerdeStructVisitor < S > { # [doc = " Completes serializing the visited object, returning `Ok(())` if all"] # [doc = " fields were serialized correctly, or `Error(S::Error)` if a field could"] # [doc = " not be serialized."] pub fn finish (self) -> Result < S :: Ok , S :: Error > { self . state ? ; self . serializer . end () } }
};
}
