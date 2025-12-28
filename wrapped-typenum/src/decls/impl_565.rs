macro_rules! deps {
    () => {
        Ord!();
        Equal!();
    };
}

macro_rules! impl_565 {
    () => {
        deps!();
        # [doc = " Returns `core::cmp::Ordering::Equal`"] impl Ord for Equal { # [inline] fn to_ordering () -> Ordering { Ordering :: Equal } }
    };
}

impl_565!()