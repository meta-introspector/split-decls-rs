macro_rules! deps {
    () => {
        TreeDesc!();
        StaticTreeDesc!();
        Value!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl < const N : usize > TreeDesc < N > { const EMPTY : Self = Self { dyn_tree : [Value :: new (0 , 0) ; N] , max_code : 0 , stat_desc : & StaticTreeDesc :: EMPTY , } ; }
    };
}

impl_178!();