macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! AsId {
    () => {
        deps!();
        # [doc = " Internal salsa trait for types that can be represented as a salsa id."] pub trait AsId : Sized { fn as_id (& self) -> Id ; }
    };
}

AsId!();