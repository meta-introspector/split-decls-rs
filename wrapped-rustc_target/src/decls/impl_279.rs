macro_rules! deps {
    () => {
        CastTarget!();
        Uniform!();
    };
}

macro_rules! impl_279 {
    () => {
        deps!();
        impl From < Uniform > for CastTarget { fn from (uniform : Uniform) -> CastTarget { Self :: prefixed ([None ; 8] , uniform) } }
    };
}

impl_279!()