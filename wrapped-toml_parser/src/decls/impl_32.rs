macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl core :: ops :: Add < Span > for usize { type Output = Span ; fn add (self , span : Span) -> Self :: Output { Self :: Output { start : span . start + self , end : span . end + self , } } }
    };
}

impl_32!();