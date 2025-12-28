macro_rules! deps {
    () => {
        FromCp437!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < 'a > FromCp437 for & 'a [u8] { type Target = :: std :: borrow :: Cow < 'a , str > ; fn from_cp437 (self) -> Self :: Target { if self . iter () . all (| c | * c < 0x80) { :: std :: str :: from_utf8 (self) . unwrap () . into () } else { self . iter () . map (| c | to_char (* c)) . collect :: < String > () . into () } } }
    };
}

impl_45!()