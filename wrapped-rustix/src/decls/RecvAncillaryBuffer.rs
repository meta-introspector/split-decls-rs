macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! RecvAncillaryBuffer {
    () => {
        deps!();
        # [doc = " Buffer for receiving ancillary messages with [`recvmsg`]."] # [doc = ""] # [doc = " Use the [`drain`] function to iterate over the received messages."] # [doc = ""] # [doc = " [`drain`]: RecvAncillaryBuffer::drain"] # [derive (Default)] pub struct RecvAncillaryBuffer < 'buf > { # [doc = " Raw byte buffer for messages."] buffer : & 'buf mut [MaybeUninit < u8 >] , # [doc = " The portion of the buffer we've read from already."] read : usize , # [doc = " The amount of the buffer that is used."] length : usize , }
    };
}

RecvAncillaryBuffer!()