macro_rules! deps {
    () => {
        Slice!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl < 'a , A : 'a , const N : usize > Index < usize > for Slice < 'a , A , N > { type Output = A ; # [inline] # [must_use] fn index (& self , index : usize) -> & Self :: Output { self . buffer . index (self . range . start + index) } }
    };
}

impl_173!();