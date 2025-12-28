macro_rules! deps {
    () => {
        IntoU64!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        # [cfg (any (target_pointer_width = "32" , target_pointer_width = "64"))] impl IntoU64 for usize { fn into_u64 (self) -> u64 { self as u64 } }
    };
}

impl_12!()