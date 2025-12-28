macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! FromId {
    () => {
        deps!();
        # [doc = " Internal Salsa trait for types that are just a newtype'd [`Id`][]."] pub trait FromId { fn from_id (id : Id) -> Self ; }
    };
}

FromId!();