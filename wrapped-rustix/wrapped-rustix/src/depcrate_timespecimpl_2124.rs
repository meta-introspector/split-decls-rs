// Generated macro for impl_2124 (impl)
macro_rules! Depcrate_timespecimpl_2124 {
() => {
// Module: crate::timespec
// Provides: {"impl_2124"}
// Dependencies: {}
impl TryFrom < Timespec > for Duration { type Error = TryFromIntError ; fn try_from (ts : Timespec) -> Result < Self , Self :: Error > { Ok (Self :: new (ts . tv_sec . try_into () ? , ts . tv_nsec as _)) } }
};
}
