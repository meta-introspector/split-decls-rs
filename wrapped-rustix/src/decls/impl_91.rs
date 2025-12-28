macro_rules! deps {
    () => {
        UserDefinedFlags!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        # [cfg (any (apple , freebsdlike))] impl UserDefinedFlags { # [doc = " Create a new `UserDefinedFlags` from a `u32`."] pub fn new (flags : u32) -> Self { Self (flags & EVFILT_USER_FLAGS) } # [doc = " Get the underlying `u32`."] pub fn get (self) -> u32 { self . 0 } }
    };
}

impl_91!()