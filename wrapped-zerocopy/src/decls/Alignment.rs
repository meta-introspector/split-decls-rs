macro_rules! Alignment {
    () => {
        # [doc = " The alignment invariant of a [`Ptr`][super::Ptr]."] pub trait Alignment : Sealed { }
    };
}

Alignment!();