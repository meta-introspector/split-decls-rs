macro_rules! deps {
    () => {
        Style!();
        SerializeValueArray!();
        Error!();
    };
}

macro_rules! impl_327 {
    () => {
        deps!();
        impl < 'd > SerializeValueArray < 'd > { pub (crate) fn seq (dst : & 'd mut String , style : Style , len : Option < usize > ,) -> Result < Self , Error > { dst . open_array () ? ; Ok (Self { dst , seen_value : false , style , len , }) } fn end (self) -> Result < & 'd mut String , Error > { if self . multiline_array () && self . seen_value { self . dst . newline () ? ; } self . dst . close_array () ? ; Ok (self . dst) } fn multiline_array (& self) -> bool { self . style . multiline_array && 2 <= self . len . unwrap_or (usize :: MAX) } }
    };
}

impl_327!();