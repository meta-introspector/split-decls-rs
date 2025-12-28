macro_rules! MMsgHdr {
    () => {
        # [doc = " An ABI-compatible wrapper for `mmsghdr`, for sending multiple messages with"] # [doc = " [sendmmsg]."] # [cfg (target_os = "linux")] # [repr (transparent)] pub struct MMsgHdr < 'a > { raw : c :: mmsghdr , _phantom : PhantomData < & 'a mut () > , }
    };
}

MMsgHdr!()