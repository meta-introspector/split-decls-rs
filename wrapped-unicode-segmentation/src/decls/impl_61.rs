macro_rules! deps {
    () => {
        UWordBounds!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < 'a > UWordBounds < 'a > { # [inline] # [doc = " View the underlying data (the part yet to be iterated) as a slice of the original string."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use unicode_segmentation::UnicodeSegmentation;"] # [doc = " let mut iter = \"Hello world\".split_word_bounds();"] # [doc = " assert_eq!(iter.as_str(), \"Hello world\");"] # [doc = " iter.next();"] # [doc = " assert_eq!(iter.as_str(), \" world\");"] # [doc = " iter.next();"] # [doc = " assert_eq!(iter.as_str(), \"world\");"] # [doc = " ```"] pub fn as_str (& self) -> & 'a str { self . string } # [inline] fn get_next_cat (& self , idx : usize) -> Option < WordCat > { use crate :: tables :: word as wd ; let nidx = idx + self . string [idx ..] . chars () . next () . unwrap () . len_utf8 () ; if nidx < self . string . len () { let nch = self . string [nidx ..] . chars () . next () . unwrap () ; Some (wd :: word_category (nch) . 2) } else { None } } # [inline] fn get_prev_cat (& self , idx : usize) -> Option < WordCat > { use crate :: tables :: word as wd ; if idx > 0 { let nch = self . string [.. idx] . chars () . next_back () . unwrap () ; Some (wd :: word_category (nch) . 2) } else { None } } }
    };
}

impl_61!()