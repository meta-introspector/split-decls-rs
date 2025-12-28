macro_rules! deps {
    () => {
        TextSize!();
        TextRange!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl TextRange { # [doc = " Creates a new `TextRange` with the given `start` and `end` (`start..end`)."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `end < start`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use text_size::*;"] # [doc = " let start = TextSize::from(5);"] # [doc = " let end = TextSize::from(10);"] # [doc = " let range = TextRange::new(start, end);"] # [doc = ""] # [doc = " assert_eq!(range.start(), start);"] # [doc = " assert_eq!(range.end(), end);"] # [doc = " assert_eq!(range.len(), end - start);"] # [doc = " ```"] # [inline] pub const fn new (start : TextSize , end : TextSize) -> TextRange { assert ! (start . raw <= end . raw) ; TextRange { start , end } } # [doc = " Create a new `TextRange` with the given `offset` and `len` (`offset..offset + len`)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use text_size::*;"] # [doc = " let text = \"0123456789\";"] # [doc = ""] # [doc = " let offset = TextSize::from(2);"] # [doc = " let length = TextSize::from(5);"] # [doc = " let range = TextRange::at(offset, length);"] # [doc = ""] # [doc = " assert_eq!(range, TextRange::new(offset, offset + length));"] # [doc = " assert_eq!(&text[range], \"23456\")"] # [doc = " ```"] # [inline] pub const fn at (offset : TextSize , len : TextSize) -> TextRange { TextRange :: new (offset , TextSize :: new (offset . raw + len . raw)) } # [doc = " Create a zero-length range at the specified offset (`offset..offset`)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use text_size::*;"] # [doc = " let point: TextSize;"] # [doc = " # point = TextSize::from(3);"] # [doc = " let range = TextRange::empty(point);"] # [doc = " assert!(range.is_empty());"] # [doc = " assert_eq!(range, TextRange::new(point, point));"] # [doc = " ```"] # [inline] pub const fn empty (offset : TextSize) -> TextRange { TextRange { start : offset , end : offset , } } # [doc = " Create a range up to the given end (`..end`)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use text_size::*;"] # [doc = " let point: TextSize;"] # [doc = " # point = TextSize::from(12);"] # [doc = " let range = TextRange::up_to(point);"] # [doc = ""] # [doc = " assert_eq!(range.len(), point);"] # [doc = " assert_eq!(range, TextRange::new(0.into(), point));"] # [doc = " assert_eq!(range, TextRange::at(0.into(), point));"] # [doc = " ```"] # [inline] pub const fn up_to (end : TextSize) -> TextRange { TextRange { start : TextSize :: new (0) , end , } } }
    };
}

impl_2!()