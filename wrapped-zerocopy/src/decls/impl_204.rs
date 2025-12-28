macro_rules! deps {
    () => {
        ConvertError!();
        SizeError!();
    };
}

macro_rules! impl_204 {
    () => {
        deps!();
        impl < Src , Dst : ? Sized , A , V > From < SizeError < Src , Dst > > for ConvertError < A , SizeError < Src , Dst > , V > { # [inline (always)] fn from (err : SizeError < Src , Dst >) -> Self { Self :: Size (err) } }
    };
}

impl_204!()