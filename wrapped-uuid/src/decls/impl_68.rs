macro_rules! deps {
    () => {
        Simple!();
        Uuid!();
        Error!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl FromStr for Simple { type Err = Error ; fn from_str (s : & str) -> Result < Self , Self :: Err > { crate :: parser :: parse_simple (s . as_bytes ()) . map (| b | Simple (Uuid (b))) . map_err (| invalid | invalid . into_err ()) } }
    };
}

impl_68!()