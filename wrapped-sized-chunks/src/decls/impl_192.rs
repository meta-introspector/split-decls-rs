macro_rules! deps {
    () => {
        SliceMut!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl < 'a , A : 'a , const N : usize > IndexMut < usize > for SliceMut < 'a , A , N > { # [inline] # [must_use] fn index_mut (& mut self , index : usize) -> & mut Self :: Output { self . buffer . index_mut (self . range . start + index) } }
    };
}

impl_192!();