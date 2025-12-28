macro_rules! Ppmd {
    () => {
        # [cfg (feature = "ppmd")] pub (crate) enum Ppmd < R : io :: BufRead > { Uninitialized (Option < R >) , Initialized (Box < ppmd_rust :: Ppmd8Decoder < R > >) , }
    };
}

Ppmd!();