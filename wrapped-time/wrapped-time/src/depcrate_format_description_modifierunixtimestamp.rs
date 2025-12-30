// Generated macro for UnixTimestamp (struct)
macro_rules! Depcrate_format_description_modifierUnixTimestamp {
() => {
// Module: crate::format_description::modifier
// Provides: {"UnixTimestamp"}
// Dependencies: {}
# [doc = " A Unix timestamp."] # [non_exhaustive] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct UnixTimestamp { # [doc = " The precision of the timestamp."] pub precision : UnixTimestampPrecision , # [doc = " Whether the `+` sign must be present for a non-negative timestamp."] pub sign_is_mandatory : bool , }
};
}
