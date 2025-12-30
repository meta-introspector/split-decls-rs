// Generated macro for impl_63 (impl)
macro_rules! Depcrate_durationimpl_63 {
() => {
// Module: crate::duration
// Provides: {"impl_63"}
// Dependencies: {}
impl TryFrom < Duration > for StdDuration { type Error = error :: ConversionRange ; # [inline] fn try_from (duration : Duration) -> Result < Self , error :: ConversionRange > { Ok (Self :: new (duration . seconds . try_into () . map_err (| _ | error :: ConversionRange) ? , duration . nanoseconds . get () . try_into () . map_err (| _ | error :: ConversionRange) ? ,)) } }
};
}
