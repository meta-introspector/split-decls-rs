macro_rules! deps {
    () => {
        SliceMut!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl < 'a , A : 'a , const N : usize > ArrayMut for SliceMut < 'a , A , N > { # [doc = " Get a mutable reference to the value at a given index."] # [inline] # [must_use] fn get_mut (& mut self , index : usize) -> Option < & mut A > { if index >= self . len () { None } else { Some (unsafe { self . get_unchecked_mut (index) }) } } }
    };
}

impl_187!()