macro_rules! deps {
    () => {
        Peek!();
    };
}

macro_rules! impl_263 {
    () => {
        deps!();
        # [cfg (feature = "parsing")] impl Peek for private :: PeekFn { type Token = private :: IdentAny ; }
    };
}

impl_263!();