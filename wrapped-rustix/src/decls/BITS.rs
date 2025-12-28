macro_rules! deps {
    () => {
        FdSetElement!();
    };
}

macro_rules! BITS {
    () => {
        deps!();
        # [cfg (not (any (windows , target_os = "wasi")))] const BITS : usize = size_of :: < FdSetElement > () * 8 ;
    };
}

BITS!();