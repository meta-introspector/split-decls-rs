macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! impl_213 {
    () => {
        deps!();
        impl < A , const N : usize > ArrayMut for RingBuffer < A , N > { # [doc = " Get a mutable reference to the value at a given index."] # [must_use] fn get_mut (& mut self , index : usize) -> Option < & mut A > { if index >= self . len () { None } else { Some (unsafe { self . get_unchecked_mut (index) }) } } }
    };
}

impl_213!();