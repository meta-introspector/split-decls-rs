macro_rules! f3 {
    () => {
        # [inline (always)] fn f3 (b : u32 , c : u32 , d : u32) -> u32 { (b & c) . wrapping_add (d & (b ^ c)) }
    };
}

f3!();