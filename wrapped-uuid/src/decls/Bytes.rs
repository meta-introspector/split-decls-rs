macro_rules! deps {
    () => {
        Uuid!();
    };
}

macro_rules! Bytes {
    () => {
        deps!();
        # [doc = " A 128-bit (16 byte) buffer containing the UUID."] # [doc = ""] # [doc = " # ABI"] # [doc = ""] # [doc = " The `Bytes` type is always guaranteed to be have the same ABI as [`Uuid`]."] pub type Bytes = [u8 ; 16] ;
    };
}

Bytes!();