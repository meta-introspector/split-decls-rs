macro_rules! borsh_support {
    () => {
        # [cfg (feature = "borsh")] pub (crate) mod borsh_support ;
    };
}

borsh_support!();