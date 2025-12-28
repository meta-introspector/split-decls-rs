macro_rules! deps {
    () => {
        Error!();
        Uuid!();
        Hyphenated!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl FromStr for Hyphenated { type Err = Error ; fn from_str (s : & str) -> Result < Self , Self :: Err > { crate :: parser :: parse_hyphenated (s . as_bytes ()) . map (| b | Hyphenated (Uuid (b))) . map_err (| invalid | invalid . into_err ()) } }
    };
}

impl_67!();