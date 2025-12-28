macro_rules! deps {
    () => {
        Unicode!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < S : AsRef < str > > Unicode < S > { pub fn to_folded_case (& self) -> String { self . 0 . as_ref () . chars () . flat_map (lookup) . collect () } }
    };
}

impl_22!();