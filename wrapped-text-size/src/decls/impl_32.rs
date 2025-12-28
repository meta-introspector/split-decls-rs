macro_rules! deps {
    () => {
        TextSize!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < A > iter :: Sum < A > for TextSize where TextSize : Add < A , Output = TextSize > , { # [inline] fn sum < I : Iterator < Item = A > > (iter : I) -> TextSize { iter . fold (0 . into () , Add :: add) } }
    };
}

impl_32!()