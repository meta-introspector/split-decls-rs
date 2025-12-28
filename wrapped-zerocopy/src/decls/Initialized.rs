macro_rules! Initialized {
    () => {
        # [doc = " The byte ranges in the referent are fully initialized. In other words, if"] # [doc = " the referent is `N` bytes long, then it contains a bit-valid `[u8; N]`."] pub enum Initialized { }
    };
}

Initialized!()