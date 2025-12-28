macro_rules! div10 {
    () => {
        # [cfg_attr (feature = "no-panic" , inline)] pub fn div10 (x : u64) -> u64 { x / 10 }
    };
}

div10!()