macro_rules! deps {
    () => {
        FromCp437!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl FromCp437 for Box < [u8] > { type Target = Box < str > ; fn from_cp437 (self) -> Self :: Target { if self . iter () . all (| c | * c < 0x80) { String :: from_utf8 (self . into ()) . unwrap () } else { self . iter () . copied () . map (to_char) . collect () } . into_boxed_str () } }
    };
}

impl_46!();