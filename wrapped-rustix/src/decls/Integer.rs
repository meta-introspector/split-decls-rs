macro_rules! Integer {
    () => {
        # [doc = " An integer that can be used by [`DecInt::new`]."] pub trait Integer : private :: Sealed { }
    };
}

Integer!()