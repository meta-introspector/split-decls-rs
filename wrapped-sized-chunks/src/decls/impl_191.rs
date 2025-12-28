macro_rules! deps {
    () => {
        SliceMut!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        impl < 'a , A : 'a , const N : usize > Index < usize > for SliceMut < 'a , A , N > { type Output = A ; # [inline] # [must_use] fn index (& self , index : usize) -> & Self :: Output { self . buffer . index (self . range . start + index) } }
    };
}

impl_191!();