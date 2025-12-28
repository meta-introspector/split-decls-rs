macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        impl < A , const N : usize > Array for RingBuffer < A , N > { # [doc = " Get a reference to the value at a given index."] # [must_use] fn get (& self , index : usize) -> Option < & A > { if index >= self . len () { None } else { Some (unsafe { self . get_unchecked (index) }) } } }
    };
}

impl_212!();