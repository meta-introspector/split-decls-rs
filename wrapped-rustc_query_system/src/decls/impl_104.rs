macro_rules! deps {
    () => {
        FingerprintStyle!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl FingerprintStyle { # [inline] pub const fn reconstructible (self) -> bool { match self { FingerprintStyle :: DefPathHash | FingerprintStyle :: Unit | FingerprintStyle :: HirId => { true } FingerprintStyle :: Opaque => false , } } }
    };
}

impl_104!()