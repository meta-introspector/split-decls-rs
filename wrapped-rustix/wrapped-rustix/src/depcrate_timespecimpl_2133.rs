// Generated macro for impl_2133 (impl)
macro_rules! Depcrate_timespecimpl_2133 {
() => {
// Module: crate::timespec
// Provides: {"impl_2133"}
// Dependencies: {}
# [cfg (fix_y2038)] impl From < Timespec > for LibcTimespec { # [inline] fn from (t : Timespec) -> Self { Self { tv_sec : t . tv_sec , tv_nsec : t . tv_nsec as _ , padding : core :: mem :: MaybeUninit :: uninit () , } } }
};
}
