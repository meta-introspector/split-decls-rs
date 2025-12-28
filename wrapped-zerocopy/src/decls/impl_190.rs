macro_rules! deps {
    () => {
        AlignmentError!();
        Unaligned!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl < Src , Dst : ? Sized + Unaligned > From < AlignmentError < Src , Dst > > for Infallible { # [inline (always)] fn from (_ : AlignmentError < Src , Dst >) -> Infallible { unsafe { core :: hint :: unreachable_unchecked () } } }
    };
}

impl_190!()