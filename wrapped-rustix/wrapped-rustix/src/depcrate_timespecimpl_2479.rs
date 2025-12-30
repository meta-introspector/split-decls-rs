// Generated macro for impl_2479 (impl)
macro_rules! Depcrate_timespecimpl_2479 {
() => {
// Module: crate::timespec
// Provides: {"impl_2479"}
// Dependencies: {}
impl TryFrom < Duration > for Timespec { type Error = TryFromIntError ; fn try_from (dur : Duration) -> Result < Self , Self :: Error > { Ok (Self { tv_sec : dur . as_secs () . try_into () ? , tv_nsec : dur . subsec_nanos () as _ , }) } }
};
}
