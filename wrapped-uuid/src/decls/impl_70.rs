macro_rules! deps {
    () => {
        Braced!();
        Uuid!();
        Error!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl FromStr for Braced { type Err = Error ; fn from_str (s : & str) -> Result < Self , Self :: Err > { crate :: parser :: parse_braced (s . as_bytes ()) . map (| b | Braced (Uuid (b))) . map_err (| invalid | invalid . into_err ()) } }
    };
}

impl_70!()