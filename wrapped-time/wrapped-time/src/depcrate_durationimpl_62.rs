// Generated macro for impl_62 (impl)
macro_rules! Depcrate_durationimpl_62 {
() => {
// Module: crate::duration
// Provides: {"impl_62"}
// Dependencies: {}
impl TryFrom < StdDuration > for Duration { type Error = error :: ConversionRange ; # [inline] fn try_from (original : StdDuration) -> Result < Self , error :: ConversionRange > { Ok (Self :: new (original . as_secs () . try_into () . map_err (| _ | error :: ConversionRange) ? , original . subsec_nanos () . cast_signed () ,)) } }
};
}
