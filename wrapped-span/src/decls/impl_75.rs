macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl Span { pub fn cover (self , other : Span) -> Span { if self . anchor != other . anchor { return self ; } let range = self . range . cover (other . range) ; Span { range , .. self } } }
    };
}

impl_75!()