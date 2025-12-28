macro_rules! is_binary {
    () => {
        # [cfg (not (feature = "detect-encoding"))] fn is_binary (_data : & [u8]) -> bool { false }
    };
}

is_binary!()