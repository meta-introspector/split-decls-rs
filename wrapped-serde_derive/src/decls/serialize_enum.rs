macro_rules! deps {
    () => {
        Container!();
        Fragment!();
        Parameters!();
        Variant!();
    };
}

macro_rules! serialize_enum {
    () => {
        deps!();
        fn serialize_enum (params : & Parameters , variants : & [Variant] , cattrs : & attr :: Container) -> Fragment { assert ! (variants . len () as u64 <= u64 :: from (u32 :: MAX)) ; let self_var = & params . self_var ; let mut arms : Vec < _ > = variants . iter () . enumerate () . map (| (variant_index , variant) | { serialize_variant (params , variant , variant_index as u32 , cattrs) }) . collect () ; if cattrs . remote () . is_some () && cattrs . non_exhaustive () { arms . push (quote ! { ref unrecognized => _serde ::# private :: Err (_serde :: ser :: Error :: custom (_serde ::# private :: ser :: CannotSerializeVariant (unrecognized))) , }) ; } quote_expr ! { match *# self_var { # (# arms) * } } }
    };
}

serialize_enum!();