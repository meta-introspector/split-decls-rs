macro_rules! deps {
    () => {
        InnerSpan!();
    };
}

macro_rules! impl_339 {
    () => {
        deps!();
        impl InnerSpan { pub fn new (start : usize , end : usize) -> InnerSpan { InnerSpan { start , end } } }
    };
}

impl_339!();