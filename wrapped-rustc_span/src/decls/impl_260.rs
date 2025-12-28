macro_rules! deps {
    () => {
        SpanData!();
        LocalDefId!();
        Span!();
        SyntaxContext!();
    };
}

macro_rules! impl_260 {
    () => {
        deps!();
        impl SpanData { # [inline] pub fn span (& self) -> Span { Span :: new (self . lo , self . hi , self . ctxt , self . parent) } # [inline] pub fn with_lo (& self , lo : BytePos) -> Span { Span :: new (lo , self . hi , self . ctxt , self . parent) } # [inline] pub fn with_hi (& self , hi : BytePos) -> Span { Span :: new (self . lo , hi , self . ctxt , self . parent) } # [doc = " Avoid if possible, `Span::map_ctxt` should be preferred."] # [inline] fn with_ctxt (& self , ctxt : SyntaxContext) -> Span { Span :: new (self . lo , self . hi , ctxt , self . parent) } # [doc = " Avoid if possible, `Span::with_parent` should be preferred."] # [inline] fn with_parent (& self , parent : Option < LocalDefId >) -> Span { Span :: new (self . lo , self . hi , self . ctxt , parent) } # [doc = " Returns `true` if this is a dummy span with any hygienic context."] # [inline] pub fn is_dummy (self) -> bool { self . lo . 0 == 0 && self . hi . 0 == 0 } # [doc = " Returns `true` if `self` fully encloses `other`."] pub fn contains (self , other : Self) -> bool { self . lo <= other . lo && other . hi <= self . hi } }
    };
}

impl_260!();