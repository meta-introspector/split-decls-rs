macro_rules! deps {
    () => {
        RawString!();
        RawStringInner!();
    };
}

macro_rules! impl_204 {
    () => {
        deps!();
        impl RawString { pub (crate) fn with_span (span : std :: ops :: Range < usize >) -> Self { Self (RawStringInner :: Spanned (span)) } # [doc = " Access the underlying string"] # [doc = ""] # [doc = " This generally requires a [`DocumentMut`][crate::DocumentMut]."] pub fn as_str (& self) -> Option < & str > { match & self . 0 { RawStringInner :: Empty => Some ("") , RawStringInner :: Explicit (s) => Some (s . as_str ()) , RawStringInner :: Spanned (_) => None , } } # [doc = " The location within the original document"] # [doc = ""] # [doc = " This generally requires a [`Document`][crate::Document]."] pub fn span (& self) -> Option < std :: ops :: Range < usize > > { match & self . 0 { RawStringInner :: Empty => None , RawStringInner :: Explicit (_) => None , RawStringInner :: Spanned (span) => Some (span . clone ()) , } } pub (crate) fn to_str < 's > (& 's self , input : & 's str) -> & 's str { match & self . 0 { RawStringInner :: Empty => "" , RawStringInner :: Explicit (s) => s . as_str () , RawStringInner :: Spanned (span) => input . get (span . clone ()) . unwrap_or_else (| | panic ! ("span {span:?} should be in input:\n```\n{input}\n```")) , } } pub (crate) fn to_str_with_default < 's > (& 's self , input : Option < & 's str > , default : & 's str ,) -> & 's str { match & self . 0 { RawStringInner :: Empty => "" , RawStringInner :: Explicit (s) => s . as_str () , RawStringInner :: Spanned (span) => { if let Some (input) = input { input . get (span . clone ()) . unwrap_or_else (| | { panic ! ("span {span:?} should be in input:\n```\n{input}\n```") }) } else { default } } } } pub (crate) fn despan (& mut self , input : & str) { match & self . 0 { RawStringInner :: Empty => { } RawStringInner :: Explicit (_) => { } RawStringInner :: Spanned (span) => { if span . start == span . end { * self = Self (RawStringInner :: Empty) ; } else { * self = Self :: from (input . get (span . clone ()) . unwrap_or_else (| | { panic ! ("span {span:?} should be in input:\n```\n{input}\n```") })) ; } } } } # [cfg (feature = "display")] pub (crate) fn encode (& self , buf : & mut dyn std :: fmt :: Write , input : & str) -> std :: fmt :: Result { let raw = self . to_str (input) ; for part in raw . split ('\r') { write ! (buf , "{part}") ? ; } Ok (()) } # [cfg (feature = "display")] pub (crate) fn encode_with_default (& self , buf : & mut dyn std :: fmt :: Write , input : Option < & str > , default : & str ,) -> std :: fmt :: Result { let raw = self . to_str_with_default (input , default) ; for part in raw . split ('\r') { write ! (buf , "{part}") ? ; } Ok (()) } }
    };
}

impl_204!()