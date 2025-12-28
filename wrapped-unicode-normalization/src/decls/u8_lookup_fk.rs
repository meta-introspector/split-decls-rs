macro_rules! u8_lookup_fk {
    () => {
        # [doc = " Extract the key in a 24 bit key and 8 bit value packed in a u32."] # [inline] fn u8_lookup_fk (kv : u32) -> u32 { kv >> 8 }
    };
}

u8_lookup_fk!();