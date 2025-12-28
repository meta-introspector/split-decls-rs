macro_rules! deps {
    () => {
        GraphemeCursor!();
        UnicodeSegmentation!();
    };
}

macro_rules! Graphemes {
    () => {
        deps!();
        # [doc = " External iterator for a string's"] # [doc = " [grapheme clusters](http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries)."] # [doc = ""] # [doc = " This struct is created by the [`graphemes`] method on the [`UnicodeSegmentation`] trait. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`graphemes`]: trait.UnicodeSegmentation.html#tymethod.graphemes"] # [doc = " [`UnicodeSegmentation`]: trait.UnicodeSegmentation.html"] # [derive (Clone , Debug)] pub struct Graphemes < 'a > { string : & 'a str , cursor : GraphemeCursor , cursor_back : GraphemeCursor , }
    };
}

Graphemes!()