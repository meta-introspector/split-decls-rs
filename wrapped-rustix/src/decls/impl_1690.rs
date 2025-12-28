macro_rules! deps {
    () => {
        Timespec!();
        LibcTimespec!();
    };
}

macro_rules! impl_1690 {
    () => {
        deps!();
        # [cfg (fix_y2038)] impl From < Timespec > for LibcTimespec { # [inline] fn from (t : Timespec) -> Self { Self { tv_sec : t . tv_sec , tv_nsec : t . tv_nsec as _ , padding : core :: mem :: MaybeUninit :: uninit () , } } }
    };
}

impl_1690!();