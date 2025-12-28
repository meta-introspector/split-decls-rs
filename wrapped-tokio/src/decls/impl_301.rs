macro_rules! deps {
    () => {
        Pointers!();
    };
}

macro_rules! impl_301 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for Pointers < T > { }
    };
}

impl_301!();