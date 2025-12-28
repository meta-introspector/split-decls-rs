macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl core :: ops :: Add < usize > for Span { type Output = Self ; fn add (self , offset : usize) -> Self :: Output { Self :: Output { start : self . start + offset , end : self . end + offset , } } }
    };
}

impl_31!();