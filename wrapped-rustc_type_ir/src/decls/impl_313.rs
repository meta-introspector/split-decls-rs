macro_rules! deps {
    () => {
        RegionFolder!();
    };
}

macro_rules! impl_313 {
    () => {
        deps!();
        impl < I , F > RegionFolder < I , F > { # [inline] pub fn new (cx : I , fold_region_fn : F) -> RegionFolder < I , F > { RegionFolder { cx , current_index : ty :: INNERMOST , fold_region_fn } } }
    };
}

impl_313!()