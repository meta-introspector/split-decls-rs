macro_rules! IMMORTAL {
    () => {
        const IMMORTAL : NonZeroUsize = NonZeroUsize :: MAX ;
    };
}

IMMORTAL!();