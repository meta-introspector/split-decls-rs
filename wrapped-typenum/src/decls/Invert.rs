macro_rules! Invert {
    () => {
        # [doc = " Converts between standard numbers and inverted ones that have the most significant"] # [doc = " digit on the outside."] pub trait Invert { type Output ; fn invert (self) -> Self :: Output ; }
    };
}

Invert!()