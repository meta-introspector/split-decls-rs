macro_rules! deps {
    () => {
        TryFromBytes!();
        Validity!();
        ValidityError!();
        ConvertError!();
    };
}

macro_rules! impl_213 {
    () => {
        deps!();
        impl < Src , Dst : ? Sized + TryFromBytes , A , S > From < ValidityError < Src , Dst > > for ConvertError < A , S , ValidityError < Src , Dst > > { # [inline (always)] fn from (err : ValidityError < Src , Dst >) -> Self { Self :: Validity (err) } }
    };
}

impl_213!()