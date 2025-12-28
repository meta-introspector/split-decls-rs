macro_rules! deps {
    () => {
        Array!();
        TinyVecIterator!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl < A : Array > Debug for TinyVecIterator < A > where A :: Item : Debug , { # [allow (clippy :: missing_inline_in_public_items)] fn fmt (& self , f : & mut Formatter < '_ >) -> core :: fmt :: Result { f . debug_tuple ("TinyVecIterator") . field (& self . as_slice ()) . finish () } }
    };
}

impl_160!()