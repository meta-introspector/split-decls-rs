macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        impl < A , const N : usize > IndexMut < usize > for RingBuffer < A , N > { # [must_use] fn index_mut (& mut self , index : usize) -> & mut Self :: Output { if index >= self . len () { panic ! ("RingBuffer::index_mut: index out of bounds {} >= {}" , index , self . len ()) ; } unsafe { & mut * self . mut_ptr (self . raw (index)) } } }
    };
}

impl_218!();