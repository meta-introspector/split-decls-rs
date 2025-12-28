macro_rules! deps {
    () => {
        DataFormat!();
        Error!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl DataFormat { # [doc = " Assumed file extension for the format"] pub fn ext (self) -> & 'static str { match self { Self :: Error => "txt" , Self :: Binary => "bin" , Self :: Text => "txt" , # [cfg (feature = "json")] Self :: Json => "json" , # [cfg (feature = "json")] Self :: JsonLines => "jsonl" , # [cfg (feature = "term-svg")] Self :: TermSvg => "term.svg" , } } }
    };
}

impl_60!();