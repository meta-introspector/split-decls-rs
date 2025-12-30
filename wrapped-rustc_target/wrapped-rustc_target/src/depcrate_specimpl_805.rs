// Generated macro for impl_805 (impl)
macro_rules! Depcrate_specimpl_805 {
() => {
// Module: crate::spec
// Provides: {"impl_805"}
// Dependencies: {}
impl < D : Decoder > Decodable < D > for TargetTuple { fn decode (d : & mut D) -> Self { match d . read_u8 () { 0 => TargetTuple :: TargetTuple (d . read_str () . to_owned ()) , 1 => TargetTuple :: TargetJson { path_for_rustdoc : PathBuf :: new () , tuple : d . read_str () . to_owned () , contents : d . read_str () . to_owned () , } , _ => { panic ! ("invalid enum variant tag while decoding `TargetTuple`, expected 0..2") ; } } } }
};
}
