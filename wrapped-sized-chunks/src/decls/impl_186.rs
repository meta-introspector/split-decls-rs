macro_rules! deps {
    () => {
        SliceMut!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl < 'a , A : 'a , const N : usize > Array for SliceMut < 'a , A , N > { # [doc = " Get a reference to the value at a given index."] # [inline] # [must_use] fn get (& self , index : usize) -> Option < & A > { if index >= self . len () { None } else { Some (unsafe { self . get_unchecked (index) }) } } }
    };
}

impl_186!();