macro_rules! deps {
    () => {
        Less!();
        Greater!();
        Equal!();
    };
}

macro_rules! Ord {
    () => {
        deps!();
        # [doc = " A **Marker trait** for the types `Greater`, `Equal`, and `Less`."] pub trait Ord : Sealed { # [allow (missing_docs)] fn to_ordering () -> :: core :: cmp :: Ordering ; }
    };
}

Ord!();