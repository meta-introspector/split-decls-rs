macro_rules! deps {
    () => {
        UnvalidatedTinyAsciiStr!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl < const N : usize > databake :: BakeSize for UnvalidatedTinyAsciiStr < N > { fn borrows_size (& self) -> usize { 0 } }
    };
}

impl_80!()