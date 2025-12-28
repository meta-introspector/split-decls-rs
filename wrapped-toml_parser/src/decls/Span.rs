macro_rules! deps {
    () => {
        Source!();
    };
}

macro_rules! Span {
    () => {
        deps!();
        # [doc = " Location within the [`Source`]"] # [derive (Copy , Clone , Default , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct Span { start : usize , end : usize , }
    };
}

Span!();