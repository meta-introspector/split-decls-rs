use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Drop for Lock { fn drop (& mut self) { let mut flock : libc :: flock = unsafe { mem :: zeroed () } ; # [cfg (not (all (target_os = "hurd" , target_arch = "x86")))] { flock . l_type = libc :: F_UNLCK as libc :: c_short ; flock . l_whence = libc :: SEEK_SET as libc :: c_short ; } # [cfg (all (target_os = "hurd" , target_arch = "x86"))] { flock . l_type = libc :: F_UNLCK as libc :: c_int ; flock . l_whence = libc :: SEEK_SET as libc :: c_int ; } flock . l_start = 0 ; flock . l_len = 0 ; unsafe { libc :: fcntl (self . file . as_raw_fd () , libc :: F_SETLK , & flock) ; } } }
}