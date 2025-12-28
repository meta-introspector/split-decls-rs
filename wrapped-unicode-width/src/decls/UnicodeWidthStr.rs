macro_rules! UnicodeWidthStr {
    () => {
        # [doc = " Methods for determining displayed width of Unicode strings."] pub trait UnicodeWidthStr : private :: Sealed { # [doc = " Returns the string's displayed width in columns."] # [doc = ""] # [doc = " This function treats characters in the Ambiguous category according"] # [doc = " to [Unicode Standard Annex #11](http://www.unicode.org/reports/tr11/)"] # [doc = " as 1 column wide. This is consistent with the recommendations for"] # [doc = " non-CJK contexts, or when the context cannot be reliably determined."] fn width (& self) -> usize ; # [doc = " Returns the string's displayed width in columns."] # [doc = ""] # [doc = " This function treats characters in the Ambiguous category according"] # [doc = " to [Unicode Standard Annex #11](http://www.unicode.org/reports/tr11/)"] # [doc = " as 2 column wide. This is consistent with the recommendations for"] # [doc = " CJK contexts."] # [cfg (feature = "cjk")] fn width_cjk (& self) -> usize ; }
    };
}

UnicodeWidthStr!()