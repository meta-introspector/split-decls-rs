macro_rules! deps {
    () => {
        DatabaseDownCaster!();
    };
}

macro_rules! impl_428 {
    () => {
        deps!();
        impl < DbView : ? Sized > Copy for DatabaseDownCaster < DbView > { }
    };
}

impl_428!();