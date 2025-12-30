// Generated macro for impl_503 (impl)
macro_rules! Depcrate_io_errnoimpl_503 {
() => {
// Module: crate::io::errno
// Provides: {"impl_503"}
// Dependencies: {}
# [cfg (feature = "std")] impl From < Errno > for std :: io :: Error { # [inline] fn from (err : Errno) -> Self { Self :: from_raw_os_error (err . raw_os_error () as _) } }
};
}
