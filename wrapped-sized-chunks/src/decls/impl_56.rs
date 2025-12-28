macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < A , const N : usize > Default for Chunk < A , N > { fn default () -> Self { Self :: new () } }
    };
}

impl_56!();