macro_rules! deps {
    () => {
        Urn!();
        Error!();
        Uuid!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl FromStr for Urn { type Err = Error ; fn from_str (s : & str) -> Result < Self , Self :: Err > { crate :: parser :: parse_urn (s . as_bytes ()) . map (| b | Urn (Uuid (b))) . map_err (| invalid | invalid . into_err ()) } }
    };
}

impl_69!();