macro_rules! deps {
    () => {
        AutoThinVec!();
        ThinVec!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        # [cfg (feature = "gecko-ffi")] impl < T , const N : usize > Deref for AutoThinVec < T , N > { type Target = ThinVec < T > ; fn deref (& self) -> & Self :: Target { & self . inner } }
    };
}

impl_89!();