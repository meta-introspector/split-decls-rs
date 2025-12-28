macro_rules! u8_lookup_fv {
    () => {
        # [doc = " Extract the value in a 24 bit key and 8 bit value packed in a u32."] # [inline] fn u8_lookup_fv (kv : u32) -> u8 { (kv & 0xff) as u8 }
    };
}

u8_lookup_fv!()