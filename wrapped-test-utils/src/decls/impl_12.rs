macro_rules! deps {
    () => {
        RangeOrOffset!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl From < RangeOrOffset > for TextRange { fn from (selection : RangeOrOffset) -> Self { match selection { RangeOrOffset :: Range (it) => it , RangeOrOffset :: Offset (it) => TextRange :: empty (it) , } } }
    };
}

impl_12!()