macro_rules! deps {
    () => {
        GraphemeIndices!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < 'a > GraphemeIndices < 'a > { # [inline] # [doc = " View the underlying data (the part yet to be iterated) as a slice of the original string."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use unicode_segmentation::UnicodeSegmentation;"] # [doc = " let mut iter = \"abc\".grapheme_indices(true);"] # [doc = " assert_eq!(iter.as_str(), \"abc\");"] # [doc = " iter.next();"] # [doc = " assert_eq!(iter.as_str(), \"bc\");"] # [doc = " iter.next();"] # [doc = " iter.next();"] # [doc = " assert_eq!(iter.as_str(), \"\");"] # [doc = " ```"] pub fn as_str (& self) -> & 'a str { self . iter . as_str () } }
    };
}

impl_2!()