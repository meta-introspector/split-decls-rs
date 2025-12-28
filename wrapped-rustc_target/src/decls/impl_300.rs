macro_rules! deps {
    () => {
        ToJson!();
    };
}

macro_rules! impl_300 {
    () => {
        deps!();
        impl < A : ToJson > ToJson for Vec < A > { fn to_json (& self) -> Json { Json :: Array (self . iter () . map (| elt | elt . to_json ()) . collect ()) } }
    };
}

impl_300!()