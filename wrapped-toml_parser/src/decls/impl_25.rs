macro_rules! deps {
    () => {
        Raw!();
        SourceIndex!();
        Source!();
        Lexer!();
        Span!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < 'i > Source < 'i > { pub fn new (input : & 'i str) -> Self { Self { input } } # [doc = " Start lexing the TOML encoded data"] pub fn lex (& self) -> Lexer < 'i > { Lexer :: new (self . input) } # [doc = " Access the TOML encoded `&str`"] pub fn input (& self) -> & 'i str { self . input } # [doc = " Return a subslice of the input"] pub fn get (& self , span : impl SourceIndex) -> Option < Raw < 'i > > { span . get (self) } # [doc = " Return an unchecked subslice of the input"] # [doc = ""] # [doc = " ## Safety"] # [doc = ""] # [doc = " Callers of this function are responsible that these preconditions are satisfied:"] # [doc = " - The starting index must not exceed the ending index;"] # [doc = " - Indexes must be within bounds of the original slice;"] # [doc = " - Indexes must lie on UTF-8 sequence boundaries."] # [doc = ""] # [doc = " Or one of:"] # [doc = " - `span` came from [`Source::lex`]"] # [doc = ""] # [doc = " Failing any of those, the returned string slice may reference invalid memory or violate the invariants communicated by `str` type."] # [cfg (feature = "unsafe")] pub unsafe fn get_unchecked (& self , span : impl SourceIndex) -> Raw < 'i > { unsafe { span . get_unchecked (self) } } # [doc = " Return a subslice of the input"] fn get_raw_str (& self , span : Span) -> Option < & 'i str > { let index = span . start () .. span . end () ; self . input . get (index) } # [doc = " Return an unchecked subslice of the input"] # [doc = ""] # [doc = " ## Safety"] # [doc = ""] # [doc = " Callers of this function are responsible that these preconditions are satisfied:"] # [doc = " - The starting index must not exceed the ending index;"] # [doc = " - Indexes must be within bounds of the original slice;"] # [doc = " - Indexes must lie on UTF-8 sequence boundaries."] # [doc = ""] # [doc = " Or one of:"] # [doc = " - `span` came from [`Source::lex`]"] # [doc = ""] # [doc = " Failing any of those, the returned string slice may reference invalid memory or violate the invariants communicated by `str` type."] # [cfg (feature = "unsafe")] unsafe fn get_raw_str_unchecked (& self , span : Span) -> & 'i str { let index = span . start () .. span . end () ; unsafe { self . input . get_unchecked (index) } } }
    };
}

impl_25!();