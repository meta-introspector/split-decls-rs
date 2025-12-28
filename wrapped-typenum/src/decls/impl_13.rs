macro_rules! deps {
    () => {
        Less!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        # [doc = " Returns `core::cmp::Ordering::Less`"] impl Ord for Less { # [inline] fn to_ordering () -> Ordering { Ordering :: Less } }
    };
}

impl_13!()