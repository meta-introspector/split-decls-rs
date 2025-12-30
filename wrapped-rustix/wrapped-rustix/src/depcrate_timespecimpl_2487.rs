// Generated macro for impl_2487 (impl)
macro_rules! Depcrate_timespecimpl_2487 {
() => {
// Module: crate::timespec
// Provides: {"impl_2487"}
// Dependencies: {}
# [cfg (fix_y2038)] impl From < Timespec > for LibcTimespec { # [inline] fn from (t : Timespec) -> Self { Self { tv_sec : t . tv_sec , tv_nsec : t . tv_nsec as _ , padding : core :: mem :: MaybeUninit :: uninit () , } } }
};
}
