macro_rules! OwnedBuf {
    () => {
        # [derive (Debug)] pub (crate) enum OwnedBuf { Vec (Vec < u8 >) , # [cfg (feature = "io-util")] Bytes (bytes :: Bytes) , }
    };
}

OwnedBuf!();