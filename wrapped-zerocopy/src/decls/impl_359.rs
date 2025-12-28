macro_rules! deps {
    () => {
        BecauseImmutable!();
        Shared!();
        SizeEq!();
        Validity!();
        TryTransmuteFromPtr!();
        Immutable!();
    };
}

macro_rules! impl_359 {
    () => {
        deps!();
        unsafe impl < Src , Dst , SV , DV > TryTransmuteFromPtr < Src , Shared , SV , DV , BecauseImmutable > for Dst where SV : Validity , DV : Validity , Src : Immutable + ? Sized , Dst : Immutable + SizeEq < Src > + ? Sized , { }
    };
}

impl_359!()