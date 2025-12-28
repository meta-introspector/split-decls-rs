macro_rules! deps {
    () => {
        DelimSpan!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < Span : Copy > DelimSpan < Span > { pub fn from_single (sp : Span) -> Self { DelimSpan { open : sp , close : sp } } pub fn from_pair (open : Span , close : Span) -> Self { DelimSpan { open , close } } }
    };
}

impl_42!();