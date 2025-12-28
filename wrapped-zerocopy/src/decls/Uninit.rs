macro_rules! Uninit {
    () => {
        # [doc = " Any bit pattern is allowed in the `Ptr`'s referent, including uninitialized"] # [doc = " bytes."] pub enum Uninit { }
    };
}

Uninit!()