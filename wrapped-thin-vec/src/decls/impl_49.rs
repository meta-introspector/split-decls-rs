macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < T > Default for ThinVec < T > { fn default () -> ThinVec < T > { ThinVec :: new () } }
    };
}

impl_49!();