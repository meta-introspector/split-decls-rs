macro_rules! deps {
    () => {
        TextRange!();
        TextSize!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl Sub < TextSize > for TextRange { type Output = TextRange ; # [inline] fn sub (self , offset : TextSize) -> TextRange { self . checked_sub (offset) . expect ("TextRange -offset overflowed") } }
    };
}

impl_13!()