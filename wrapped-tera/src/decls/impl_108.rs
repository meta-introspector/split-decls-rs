macro_rules! deps {
    () => {
        PointerMachina!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl PointerMachina < '_ > { fn new (pointer : & str) -> PointerMachina < '_ > { PointerMachina { pointer , single_quoted : false , dual_quoted : false , escaped : false , last_position : 0 , } } }
    };
}

impl_108!()