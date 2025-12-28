macro_rules! deps {
    () => {
        InlineSize!();
    };
}

macro_rules! INLINE_CAP {
    () => {
        deps!();
        const INLINE_CAP : usize = InlineSize :: _V23 as usize ;
    };
}

INLINE_CAP!();