macro_rules! deps {
    () => {
        ToJson!();
    };
}

macro_rules! impl_475 {
    () => {
        deps!();
        impl ToJson for LinkSelfContainedComponents { fn to_json (& self) -> Json { let components : Vec < _ > = Self :: all_components () . into_iter () . filter (| c | self . contains (* c)) . map (| c | { c . as_str () . unwrap () . to_owned () }) . collect () ; components . to_json () } }
    };
}

impl_475!()