macro_rules! deps {
    () => {
        ToJson!();
    };
}

macro_rules! impl_299 {
    () => {
        deps!();
        impl < A : ToJson > ToJson for [A] { fn to_json (& self) -> Json { Json :: Array (self . iter () . map (| elt | elt . to_json ()) . collect ()) } }
    };
}

impl_299!();