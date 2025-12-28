macro_rules! deps {
    () => {
        TextSize!();
        TextRange!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl Add < TextSize > for TextRange { type Output = TextRange ; # [inline] fn add (self , offset : TextSize) -> TextRange { self . checked_add (offset) . expect ("TextRange +offset overflowed") } }
    };
}

impl_12!();