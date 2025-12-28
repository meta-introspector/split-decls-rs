macro_rules! Sha1Core {
    () => {
        # [doc = " Core SHA-1 hasher state."] # [derive (Clone)] pub struct Sha1Core { h : [u32 ; STATE_LEN] , block_len : u64 , }
    };
}

Sha1Core!()