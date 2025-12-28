macro_rules! deps {
    () => {
        Background!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl Background { fn attr (self) -> & 'static str { match self { Self :: Dark => "bgcolor=\"#f0f0f0\"" , Self :: Light => "" , } } }
    };
}

impl_75!()