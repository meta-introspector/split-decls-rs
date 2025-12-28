macro_rules! EVFILT_USER_FLAGS {
    () => {
        # [doc = " Bottom 24 bits of a `u32`."] # [cfg (any (apple , freebsdlike))] const EVFILT_USER_FLAGS : u32 = 0x00ff_ffff ;
    };
}

EVFILT_USER_FLAGS!();