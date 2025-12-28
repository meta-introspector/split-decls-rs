macro_rules! deps {
    () => {
        InactiveVariants!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl InactiveVariants { fn contains (& self , variant_idx : VariantIdx) -> bool { match self { InactiveVariants :: Inactives (inactives) => inactives . contains (& variant_idx) , InactiveVariants :: Active (active) => variant_idx != * active , } } }
    };
}

impl_13!()