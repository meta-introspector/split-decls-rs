macro_rules! f1 {
    () => {
        # [inline (always)] fn f1 (b : u32 , c : u32 , d : u32) -> u32 { d ^ b & (c ^ d) }
    };
}

f1!()