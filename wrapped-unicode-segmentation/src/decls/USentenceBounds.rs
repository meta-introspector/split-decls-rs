macro_rules! deps {
    () => {
        UnicodeSegmentation!();
    };
}

macro_rules! USentenceBounds {
    () => {
        deps!();
        # [doc = " External iterator for a string's"] # [doc = " [sentence boundaries](http://www.unicode.org/reports/tr29/#Sentence_Boundaries)."] # [doc = ""] # [doc = " This struct is created by the [`split_sentence_bounds`] method on the [`UnicodeSegmentation`]"] # [doc = " trait. See its documentation for more."] # [doc = ""] # [doc = " [`split_sentence_bounds`]: trait.UnicodeSegmentation.html#tymethod.split_sentence_bounds"] # [doc = " [`UnicodeSegmentation`]: trait.UnicodeSegmentation.html"] # [derive (Debug , Clone)] pub struct USentenceBounds < 'a > { iter : fwd :: SentenceBreaks < 'a > , sentence_start : Option < usize > , }
    };
}

USentenceBounds!()