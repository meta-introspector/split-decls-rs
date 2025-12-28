macro_rules! deps {
    () => {
        Latch!();
        LatchRef!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < L : Latch > Latch for LatchRef < '_ , L > { # [inline] unsafe fn set (this : * const Self) { unsafe { L :: set ((* this) . inner) } ; } }
    };
}

impl_96!()