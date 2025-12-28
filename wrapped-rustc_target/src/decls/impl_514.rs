macro_rules! deps {
    () => {
        SanitizerSet!();
        ToJson!();
    };
}

macro_rules! impl_514 {
    () => {
        deps!();
        impl ToJson for SanitizerSet { fn to_json (& self) -> Json { self . into_iter () . map (| v | Some (v . as_str () ? . to_json ())) . collect :: < Option < Vec < _ > > > () . unwrap_or_default () . to_json () } }
    };
}

impl_514!();