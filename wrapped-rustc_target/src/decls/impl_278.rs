macro_rules! deps {
    () => {
        CastTarget!();
        Uniform!();
    };
}

macro_rules! impl_278 {
    () => {
        deps!();
        impl From < Reg > for CastTarget { fn from (unit : Reg) -> CastTarget { CastTarget :: from (Uniform :: from (unit)) } }
    };
}

impl_278!();