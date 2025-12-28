macro_rules! deps {
    () => {
        Timespec!();
        LibcTimespec!();
    };
}

macro_rules! impl_1689 {
    () => {
        deps!();
        # [cfg (fix_y2038)] impl From < LibcTimespec > for Timespec { # [inline] fn from (t : LibcTimespec) -> Self { Self { tv_sec : t . tv_sec , tv_nsec : t . tv_nsec as _ , } } }
    };
}

impl_1689!()