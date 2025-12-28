macro_rules! deps {
    () => {
        UnicodeSegmentation!();
        WordsIter!();
    };
}

macro_rules! UnicodeWords {
    () => {
        deps!();
        # [doc = " An iterator over the substrings of a string which, after splitting the string on"] # [doc = " [word boundaries](http://www.unicode.org/reports/tr29/#Word_Boundaries),"] # [doc = " contain any characters with the"] # [doc = " [Alphabetic](http://unicode.org/reports/tr44/#Alphabetic)"] # [doc = " property, or with"] # [doc = " [General_Category=Number](http://unicode.org/reports/tr44/#General_Category_Values)."] # [doc = ""] # [doc = " This struct is created by the [`unicode_words`] method on the [`UnicodeSegmentation`] trait. See"] # [doc = " its documentation for more."] # [doc = ""] # [doc = " [`unicode_words`]: trait.UnicodeSegmentation.html#tymethod.unicode_words"] # [doc = " [`UnicodeSegmentation`]: trait.UnicodeSegmentation.html"] # [derive (Debug)] pub struct UnicodeWords < 'a > { inner : WordsIter < 'a > , }
    };
}

UnicodeWords!()