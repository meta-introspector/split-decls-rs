macro_rules! f4 {
    () => {
        # [inline (always)] fn f4 (b : u32 , c : u32 , d : u32) -> u32 { b ^ c ^ d }
    };
}

f4!()