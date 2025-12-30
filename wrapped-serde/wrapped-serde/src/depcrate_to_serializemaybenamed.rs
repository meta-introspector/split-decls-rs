// Generated macro for MaybeNamed (enum)
macro_rules! Depcrate_to_serializeMaybeNamed {
() => {
// Module: crate::to_serialize
// Provides: {"MaybeNamed"}
// Dependencies: {}
enum MaybeNamed < TNamed , TUnnamed > { Named { serializer : TNamed } , Unnamed { serializer : TUnnamed } , }
};
}
