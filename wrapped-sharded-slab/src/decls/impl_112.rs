macro_rules! deps {
    () => {
        LifecycleGen!();
        Generation!();
        Config!();
        RefCount!();
        Pack!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl < C : cfg :: Config > Pack < C > for LifecycleGen < C > { const LEN : usize = Generation :: < C > :: LEN ; type Prev = RefCount < C > ; fn from_usize (value : usize) -> Self { Self (Generation :: from_usize (value)) } fn as_usize (& self) -> usize { self . 0 . as_usize () } }
    };
}

impl_112!()