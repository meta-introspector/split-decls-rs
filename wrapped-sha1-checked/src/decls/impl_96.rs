macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl Default for Builder { fn default () -> Self { Self { detect_collision : true , safe_hash : true , ubc_check : true , reduced_round_collision : false , } } }
    };
}

impl_96!()