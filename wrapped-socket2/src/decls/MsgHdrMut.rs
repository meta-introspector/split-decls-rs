macro_rules! deps {
    () => {
        SockAddr!();
        MaybeUninitSlice!();
        MsgHdr!();
    };
}

macro_rules! MsgHdrMut {
    () => {
        deps!();
        # [doc = " Configuration of a `recvmsg(2)` system call."] # [doc = ""] # [doc = " This wraps `msghdr` on Unix and `WSAMSG` on Windows. Also see [`MsgHdr`] for"] # [doc = " the variant used by `sendmsg(2)`."] # [cfg (not (target_os = "redox"))] pub struct MsgHdrMut < 'addr , 'bufs , 'control > { inner : sys :: msghdr , # [allow (clippy :: type_complexity)] _lifetimes : PhantomData < (& 'addr mut SockAddr , & 'bufs mut MaybeUninitSlice < 'bufs > , & 'control mut [u8] ,) > , }
    };
}

MsgHdrMut!();