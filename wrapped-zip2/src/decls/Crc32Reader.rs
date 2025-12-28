macro_rules! Crc32Reader {
    () => {
        # [doc = " Reader that validates the CRC32 when it reaches the EOF."] pub struct Crc32Reader < R > { inner : R , hasher : Hasher , check : u32 , # [doc = " Signals if `inner` stores aes encrypted data."] # [doc = " AE-2 encrypted data doesn't use crc and sets the value to 0."] enabled : bool , }
    };
}

Crc32Reader!();