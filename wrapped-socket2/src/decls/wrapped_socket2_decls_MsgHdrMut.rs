use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Configuration of a `recvmsg(2)` system call.
///
/// This wraps `msghdr` on Unix and `WSAMSG` on Windows. Also see [`MsgHdr`] for
/// the variant used by `sendmsg(2)`.
#[cfg(not(target_os = "redox"))]
pub struct MsgHdrMut<'addr, 'bufs, 'control> {
    inner: sys::msghdr,
    #[allow(clippy::type_complexity)]
    _lifetimes: PhantomData<(
        &'addr mut SockAddr,
        &'bufs mut MaybeUninitSlice<'bufs>,
        &'control mut [u8],
    )>,
}
