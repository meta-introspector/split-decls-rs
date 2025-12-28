macro_rules! deps {
    () => {
        Raw!();
        Source!();
    };
}

macro_rules! SourceIndex {
    () => {
        deps!();
        # [doc = " A helper trait used for indexing operations on [`Source`]"] pub trait SourceIndex : sealed :: Sealed { # [doc = " Return a subslice of the input"] fn get < 'i > (self , source : & Source < 'i >) -> Option < Raw < 'i > > ; # [doc = " Return an unchecked subslice of the input"] # [doc = ""] # [doc = " ## Safety"] # [doc = ""] # [doc = " Callers of this function are responsible that these preconditions are satisfied:"] # [doc = " - The starting index must not exceed the ending index;"] # [doc = " - Indexes must be within bounds of the original slice;"] # [doc = " - Indexes must lie on UTF-8 sequence boundaries."] # [doc = ""] # [doc = " Or one of:"] # [doc = " - `span` came from [`Source::lex`]"] # [doc = ""] # [doc = " Failing any of those, the returned string slice may reference invalid memory or violate the invariants communicated by `str` type."] # [cfg (feature = "unsafe")] unsafe fn get_unchecked < 'i > (self , source : & Source < 'i >) -> Raw < 'i > ; }
    };
}

SourceIndex!();