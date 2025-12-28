macro_rules! deps {
    () => {
        SliceKind!();
    };
}

macro_rules! Slice {
    () => {
        deps!();
        # [doc = " A constructor for array and slice patterns."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub struct Slice { # [doc = " `None` if the matched value is a slice, `Some(n)` if it is an array of size `n`."] pub (crate) array_len : Option < usize > , # [doc = " The kind of pattern it is: fixed-length `[x, y]` or variable length `[x, .., y]`."] pub (crate) kind : SliceKind , }
    };
}

Slice!();