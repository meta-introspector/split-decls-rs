macro_rules! impl_433 {
    () => {
        impl LitBool { pub fn new (value : bool , span : Span) -> Self { LitBool { value , span } } pub fn value (& self) -> bool { self . value } pub fn span (& self) -> Span { self . span } pub fn set_span (& mut self , span : Span) { self . span = span ; } pub fn token (& self) -> Ident { let s = if self . value { "true" } else { "false" } ; Ident :: new (s , self . span) } }
    };
}

impl_433!();