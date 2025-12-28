macro_rules! deps {
    () => {
        Ord!();
        Greater!();
    };
}

macro_rules! impl_563 {
    () => {
        deps!();
        # [doc = " Returns `core::cmp::Ordering::Greater`"] impl Ord for Greater { # [inline] fn to_ordering () -> Ordering { Ordering :: Greater } }
    };
}

impl_563!();