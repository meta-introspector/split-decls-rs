macro_rules! deps {
    () => {
        TryCastError!();
        Alignment!();
        Validity!();
        CastError!();
        TryFromBytes!();
    };
}

macro_rules! impl_219 {
    () => {
        deps!();
        impl < Src , Dst : ? Sized + TryFromBytes > From < CastError < Src , Dst > > for TryCastError < Src , Dst > { # [inline] fn from (value : CastError < Src , Dst >) -> Self { match value { CastError :: Alignment (e) => Self :: Alignment (e) , CastError :: Size (e) => Self :: Size (e) , CastError :: Validity (i) => match i { } , } } }
    };
}

impl_219!();