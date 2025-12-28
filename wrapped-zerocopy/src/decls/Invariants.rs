macro_rules! deps {
    () => {
        Alignment!();
        Aliasing!();
        Validity!();
    };
}

macro_rules! Invariants {
    () => {
        deps!();
        # [doc = " The invariants of a [`Ptr`][super::Ptr]."] pub trait Invariants : Sealed { type Aliasing : Aliasing ; type Alignment : Alignment ; type Validity : Validity ; }
    };
}

Invariants!();