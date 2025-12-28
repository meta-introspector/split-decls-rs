macro_rules! deps {
    () => {
        Trait!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl Trait { fn crate_path (& self , zerocopy_crate : & Path) -> Path { match self { Self :: Sized => { parse_quote ! (# zerocopy_crate :: util :: macro_util :: core_reexport :: marker ::# self) } _ => parse_quote ! (# zerocopy_crate ::# self) , } } }
    };
}

impl_109!();