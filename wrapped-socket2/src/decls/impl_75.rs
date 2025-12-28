macro_rules! deps {
    () => {
        MsgHdrMut!();
        MaybeUninitSlice!();
        RecvFlags!();
        SockAddr!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        # [cfg (not (target_os = "redox"))] impl < 'addr , 'bufs , 'control > MsgHdrMut < 'addr , 'bufs , 'control > { # [doc = " Create a new `MsgHdrMut` with all empty/zero fields."] # [allow (clippy :: new_without_default)] pub fn new () -> MsgHdrMut < 'addr , 'bufs , 'control > { MsgHdrMut { inner : unsafe { mem :: zeroed () } , _lifetimes : PhantomData , } } # [doc = " Set the mutable address (name) of the message."] # [doc = ""] # [doc = " Corresponds to setting `msg_name` and `msg_namelen` on Unix and `name`"] # [doc = " and `namelen` on Windows."] # [allow (clippy :: needless_pass_by_ref_mut)] pub fn with_addr (mut self , addr : & 'addr mut SockAddr) -> Self { sys :: set_msghdr_name (& mut self . inner , addr) ; self } # [doc = " Set the mutable buffer(s) of the message."] # [doc = ""] # [doc = " Corresponds to setting `msg_iov` and `msg_iovlen` on Unix and `lpBuffers`"] # [doc = " and `dwBufferCount` on Windows."] pub fn with_buffers (mut self , bufs : & 'bufs mut [MaybeUninitSlice < '_ >]) -> Self { sys :: set_msghdr_iov (& mut self . inner , bufs . as_mut_ptr () . cast () , bufs . len ()) ; self } # [doc = " Set the mutable control buffer of the message."] # [doc = ""] # [doc = " Corresponds to setting `msg_control` and `msg_controllen` on Unix and"] # [doc = " `Control` on Windows."] pub fn with_control (mut self , buf : & 'control mut [MaybeUninit < u8 >]) -> Self { sys :: set_msghdr_control (& mut self . inner , buf . as_mut_ptr () . cast () , buf . len ()) ; self } # [doc = " Returns the flags of the message."] pub fn flags (& self) -> RecvFlags { sys :: msghdr_flags (& self . inner) } # [doc = " Gets the length of the control buffer."] # [doc = ""] # [doc = " Can be used to determine how much, if any, of the control buffer was filled by `recvmsg`."] # [doc = ""] # [doc = " Corresponds to `msg_controllen` on Unix and `Control.len` on Windows."] pub fn control_len (& self) -> usize { sys :: msghdr_control_len (& self . inner) } }
    };
}

impl_75!();