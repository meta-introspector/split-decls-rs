macro_rules! deps {
    () => {
        OneShot!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl OneShot { fn parse_toml (s : & str) -> Result < Self , crate :: Error > { toml_edit :: de :: from_str (s) . map_err (| e | e . to_string () . into ()) } }
    };
}

impl_14!();