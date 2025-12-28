macro_rules! deps {
    () => {
        UniCase!();
        Ascii!();
        Unicode!();
        Encoding!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < S : AsRef < str > > UniCase < S > { # [doc = " Creates a new `UniCase`."] # [doc = ""] # [doc = " Note: This scans the text to determine if it is all ASCII or not."] pub fn new (s : S) -> UniCase < S > { if s . as_ref () . is_ascii () { UniCase (Encoding :: Ascii (Ascii (s))) } else { UniCase (Encoding :: Unicode (Unicode (s))) } } # [doc = " Returns a copy of this string where each character is mapped to its"] # [doc = " Unicode CaseFolding equivalent."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " Unicode Case Folding is meant for string storage and matching, not for"] # [doc = " display."] pub fn to_folded_case (& self) -> String { match self . 0 { Encoding :: Ascii (ref s) => s . 0 . as_ref () . to_ascii_lowercase () , Encoding :: Unicode (ref s) => s . to_folded_case () , } } }
    };
}

impl_39!();