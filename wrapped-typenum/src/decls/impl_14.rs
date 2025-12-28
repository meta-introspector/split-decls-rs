macro_rules! deps {
    () => {
        Equal!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        # [doc = " Returns `core::cmp::Ordering::Equal`"] impl Ord for Equal { # [inline] fn to_ordering () -> Ordering { Ordering :: Equal } }
    };
}

impl_14!()