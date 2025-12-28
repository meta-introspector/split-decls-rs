macro_rules! Halves {
    () => {
        pub trait Halves { type Output ; fn upper_half (self) -> Self :: Output ; fn lower_half (self) -> Self :: Output ; }
    };
}

Halves!()