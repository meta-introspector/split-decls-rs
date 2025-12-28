macro_rules! Crc32Fold {
    () => {
        # [derive (Debug , Clone , Copy)] pub struct Crc32Fold { # [cfg (target_arch = "x86_64")] fold : pclmulqdq :: Accumulator , value : u32 , }
    };
}

Crc32Fold!()