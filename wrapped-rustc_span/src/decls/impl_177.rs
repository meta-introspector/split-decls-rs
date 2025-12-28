macro_rules! deps {
    () => {
        Span!();
        Ident!();
        Symbol!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        impl Ident { # [inline] # [doc = " Constructs a new identifier from a symbol and a span."] pub fn new (name : Symbol , span : Span) -> Ident { debug_assert_ne ! (name , sym :: empty) ; Ident { name , span } } # [doc = " Constructs a new identifier with a dummy span."] # [inline] pub fn with_dummy_span (name : Symbol) -> Ident { Ident :: new (name , DUMMY_SP) } # [inline] pub fn dummy () -> Ident { Ident :: with_dummy_span (sym :: dummy) } # [doc = " Maps a string to an identifier with a dummy span."] pub fn from_str (string : & str) -> Ident { Ident :: with_dummy_span (Symbol :: intern (string)) } # [doc = " Maps a string and a span to an identifier."] pub fn from_str_and_span (string : & str , span : Span) -> Ident { Ident :: new (Symbol :: intern (string) , span) } # [doc = " Replaces `lo` and `hi` with those from `span`, but keep hygiene context."] pub fn with_span_pos (self , span : Span) -> Ident { Ident :: new (self . name , span . with_ctxt (self . span . ctxt ())) } pub fn without_first_quote (self) -> Ident { Ident :: new (Symbol :: intern (self . as_str () . trim_start_matches ('\'')) , self . span) } # [doc = " \"Normalize\" ident for use in comparisons using \"item hygiene\"."] # [doc = " Identifiers with same string value become same if they came from the same macro 2.0 macro"] # [doc = " (e.g., `macro` item, but not `macro_rules` item) and stay different if they came from"] # [doc = " different macro 2.0 macros."] # [doc = " Technically, this operation strips all non-opaque marks from ident's syntactic context."] pub fn normalize_to_macros_2_0 (self) -> Ident { Ident :: new (self . name , self . span . normalize_to_macros_2_0 ()) } # [doc = " \"Normalize\" ident for use in comparisons using \"local variable hygiene\"."] # [doc = " Identifiers with same string value become same if they came from the same non-transparent"] # [doc = " macro (e.g., `macro` or `macro_rules!` items) and stay different if they came from different"] # [doc = " non-transparent macros."] # [doc = " Technically, this operation strips all transparent marks from ident's syntactic context."] # [inline] pub fn normalize_to_macro_rules (self) -> Ident { Ident :: new (self . name , self . span . normalize_to_macro_rules ()) } # [doc = " Access the underlying string. This is a slowish operation because it"] # [doc = " requires locking the symbol interner."] # [doc = ""] # [doc = " Note that the lifetime of the return value is a lie. See"] # [doc = " `Symbol::as_str()` for details."] pub fn as_str (& self) -> & str { self . name . as_str () } }
    };
}

impl_177!();