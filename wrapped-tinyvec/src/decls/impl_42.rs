macro_rules! deps {
    () => {
        ArrayVecIterator!();
        Array!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < A : Array > ArrayVecIterator < A > { # [doc = " Returns the remaining items of this iterator as a slice."] # [inline] # [must_use] pub fn as_slice (& self) -> & [A :: Item] { & self . data . as_slice () [self . base as usize .. self . tail as usize] } }
    };
}

impl_42!()