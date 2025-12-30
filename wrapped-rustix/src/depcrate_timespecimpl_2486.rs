// Generated macro for impl_2486 (impl)
macro_rules! Depcrate_timespecimpl_2486 {
() => {
// Module: crate::timespec
// Provides: {"impl_2486"}
// Dependencies: {}
# [cfg (fix_y2038)] impl From < LibcTimespec > for Timespec { # [inline] fn from (t : LibcTimespec) -> Self { Self { tv_sec : t . tv_sec , tv_nsec : t . tv_nsec as _ , } } }
};
}
