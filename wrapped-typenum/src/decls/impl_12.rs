macro_rules! deps {
    () => {
        Greater!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        # [doc = " Returns `core::cmp::Ordering::Greater`"] impl Ord for Greater { # [inline] fn to_ordering () -> Ordering { Ordering :: Greater } }
    };
}

impl_12!()