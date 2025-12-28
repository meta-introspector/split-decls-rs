macro_rules! impl_296 {
    () => {
        # [cfg (feature = "std")] impl From < Errno > for std :: io :: Error { # [inline] fn from (err : Errno) -> Self { Self :: from_raw_os_error (err . raw_os_error () as _) } }
    };
}

impl_296!();