macro_rules! deps {
    () => {
        TinyAsciiStr!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < const N : usize > BakeSize for TinyAsciiStr < N > { fn borrows_size (& self) -> usize { 0 } }
    };
}

impl_78!()