macro_rules! deps {
    () => {
        SockAddr!();
        MsgHdrMut!();
    };
}

macro_rules! MsgHdr {
    () => {
        deps!();
        # [doc = " Configuration of a `sendmsg(2)` system call."] # [doc = ""] # [doc = " This wraps `msghdr` on Unix and `WSAMSG` on Windows. Also see [`MsgHdrMut`]"] # [doc = " for the variant used by `recvmsg(2)`."] # [cfg (not (target_os = "redox"))] pub struct MsgHdr < 'addr , 'bufs , 'control > { inner : sys :: msghdr , # [allow (clippy :: type_complexity)] _lifetimes : PhantomData < (& 'addr SockAddr , & 'bufs IoSlice < 'bufs > , & 'control [u8]) > , }
    };
}

MsgHdr!();