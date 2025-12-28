macro_rules! deps {
    () => {
        TinyAsciiStr!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl < const N : usize > NicheBytes < N > for TinyAsciiStr < N > { const NICHE_BIT_PATTERN : [u8 ; N] = [255 ; N] ; }
    };
}

impl_85!()