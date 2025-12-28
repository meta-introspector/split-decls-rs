macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! impl_217 {
    () => {
        deps!();
        impl < A , const N : usize > Index < usize > for RingBuffer < A , N > { type Output = A ; # [must_use] fn index (& self , index : usize) -> & Self :: Output { if index >= self . len () { panic ! ("RingBuffer::index: index out of bounds {} >= {}" , index , self . len ()) ; } unsafe { & * self . ptr (self . raw (index)) } } }
    };
}

impl_217!()