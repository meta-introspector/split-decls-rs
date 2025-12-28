macro_rules! deps {
    () => {
        AlignmentError!();
        ConvertError!();
        Alignment!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        impl < Src , Dst : ? Sized , S , V > From < AlignmentError < Src , Dst > > for ConvertError < AlignmentError < Src , Dst > , S , V > { # [inline (always)] fn from (err : AlignmentError < Src , Dst >) -> Self { Self :: Alignment (err) } }
    };
}

impl_195!()