// Generated macro for system_time_to_cfdate (function)
macro_rules! Depcrate_verification_applesystem_time_to_cfdate {
() => {
// Module: crate::verification::apple
// Provides: {"system_time_to_cfdate"}
// Dependencies: {}
# [allow (clippy :: as_conversions)] fn system_time_to_cfdate (time : pki_types :: UnixTime) -> Result < CFDate , TlsError > { let unix_adjustment = unsafe { kCFAbsoluteTimeIntervalSince1970 as u64 } ; time . as_secs () . checked_sub (unix_adjustment) . ok_or (TlsError :: FailedToGetCurrentTime) . map (| epoch | CFDate :: new (epoch as f64)) }
};
}
