macro_rules! deps {
    () => {
        Slice!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl < 'a , A : 'a , const N : usize > Array for Slice < 'a , A , N > { # [doc = " Get a reference to the value at a given index."] # [inline] # [must_use] fn get (& self , index : usize) -> Option < & A > { if index >= self . len () { None } else { Some (unsafe { self . get_unchecked (index) }) } } }
    };
}

impl_169!();