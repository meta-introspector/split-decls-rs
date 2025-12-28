macro_rules! deps {
    () => {
        CpuSet!();
    };
}

macro_rules! impl_1374 {
    () => {
        deps!();
        impl hash :: Hash for CpuSet { fn hash < H : hash :: Hasher > (& self , state : & mut H) { for i in 0 .. Self :: MAX_CPU { self . is_set (i) . hash (state) ; } } }
    };
}

impl_1374!()