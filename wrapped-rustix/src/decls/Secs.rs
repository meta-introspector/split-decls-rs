macro_rules! deps {
    () => {
        Timespec!();
    };
}

macro_rules! Secs {
    () => {
        deps!();
        # [doc = " A type for the `tv_sec` field of [`Timespec`]."] pub type Secs = i64 ;
    };
}

Secs!();