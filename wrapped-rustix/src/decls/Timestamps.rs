macro_rules! deps {
    () => {
        Timespec!();
    };
}

macro_rules! Timestamps {
    () => {
        deps!();
        # [doc = " Timestamps used by [`utimensat`] and [`futimens`]."] # [doc = ""] # [doc = " [`utimensat`]: crate::fs::utimensat"] # [doc = " [`futimens`]: crate::fs::futimens"] # [repr (C)] # [derive (Debug , Clone)] pub struct Timestamps { # [doc = " The timestamp of the last access to a filesystem object."] pub last_access : Timespec , # [doc = " The timestamp of the last modification of a filesystem object."] pub last_modification : Timespec , }
    };
}

Timestamps!();