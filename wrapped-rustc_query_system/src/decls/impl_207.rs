macro_rules! deps {
    () => {
        QueryStackFrameExtra!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        impl QueryStackFrameExtra { # [inline] pub fn new (description : String , span : Option < Span > , def_kind : Option < DefKind >) -> Self { Self { description , span , def_kind } } # [inline] pub fn default_span (& self , span : Span) -> Span { if ! span . is_dummy () { return span ; } self . span . unwrap_or (span) } }
    };
}

impl_207!();