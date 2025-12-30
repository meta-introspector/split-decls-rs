// Generated macro for impl_2132 (impl)
macro_rules! Depcrate_timespecimpl_2132 {
() => {
// Module: crate::timespec
// Provides: {"impl_2132"}
// Dependencies: {}
# [cfg (fix_y2038)] impl From < LibcTimespec > for Timespec { # [inline] fn from (t : LibcTimespec) -> Self { Self { tv_sec : t . tv_sec , tv_nsec : t . tv_nsec as _ , } } }
};
}
