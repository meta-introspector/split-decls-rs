macro_rules! deps {
    () => {
        Ord!();
        Less!();
    };
}

macro_rules! impl_564 {
    () => {
        deps!();
        # [doc = " Returns `core::cmp::Ordering::Less`"] impl Ord for Less { # [inline] fn to_ordering () -> Ordering { Ordering :: Less } }
    };
}

impl_564!();