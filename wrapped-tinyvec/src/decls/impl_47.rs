macro_rules! deps {
    () => {
        ArrayVecIterator!();
        Array!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < A : Array > Debug for ArrayVecIterator < A > where A :: Item : Debug , { # [allow (clippy :: missing_inline_in_public_items)] fn fmt (& self , f : & mut Formatter < '_ >) -> core :: fmt :: Result { f . debug_tuple ("ArrayVecIterator") . field (& self . as_slice ()) . finish () } }
    };
}

impl_47!()