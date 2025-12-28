macro_rules! f2 {
    () => {
        # [inline (always)] fn f2 (b : u32 , c : u32 , d : u32) -> u32 { b ^ c ^ d }
    };
}

f2!();