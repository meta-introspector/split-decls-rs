macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! impl_216 {
    () => {
        deps!();
        impl < A : Clone , const N : usize > Clone for RingBuffer < A , N > { fn clone (& self) -> Self { let mut out = Self :: new () ; out . origin = self . origin ; out . length = self . length ; let range = self . range () ; out . length = 0 ; for index in range { unsafe { out . force_write (index , (& * self . ptr (index)) . clone ()) } ; out . length += 1 ; } out } }
    };
}

impl_216!();