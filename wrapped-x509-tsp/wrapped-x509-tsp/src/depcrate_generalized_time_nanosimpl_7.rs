// Generated macro for impl_7 (impl)
macro_rules! Depcrate_generalized_time_nanosimpl_7 {
() => {
// Module: crate::generalized_time_nanos
// Provides: {"impl_7"}
// Dependencies: {}
impl GeneralizedTimeNanos { # [doc = " Length of an RFC 5280-flavored ASN.1 DER-encoded [`GeneralizedTimeNanos`]."] const MIN_LENGTH : usize = 15 ; # [doc = " Maximum length of a GeneralizedTime containing nanoseconds."] const MAX_LENGTH : usize = Self :: MIN_LENGTH + 10 ; # [doc = " Get the duration of this timestamp since `UNIX_EPOCH`."] pub fn to_unix_duration (& self) -> Duration { self . datetime . unix_duration () + Duration :: from_nanos (u64 :: from (self . nanoseconds)) } # [doc = " Create a new [`GeneralizedTimeNanos`] given a [`Duration`] since"] # [doc = " `UNIX_EPOCH` (a.k.a. \"Unix time\")"] pub fn from_unix_duration (unix_duration : Duration) -> Result < Self > { let datetime = DateTime :: from_unix_duration (unix_duration) . map_err (| _ | Self :: TAG . value_error ()) ? ; Ok (GeneralizedTimeNanos { datetime , nanoseconds : unix_duration . subsec_nanos () , }) } }
};
}
