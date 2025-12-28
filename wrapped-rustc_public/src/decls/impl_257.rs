macro_rules! deps {
    () => {
        Attribute!();
        Span!();
    };
}

macro_rules! impl_257 {
    () => {
        deps!();
        impl Attribute { pub fn new (value : String , span : Span) -> Attribute { Attribute { value , span } } # [doc = " Get the span of this attribute."] pub fn span (& self) -> Span { self . span } # [doc = " Get the string representation of this attribute."] pub fn as_str (& self) -> & str { & self . value } }
    };
}

impl_257!()