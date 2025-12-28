macro_rules! deps {
    () => {
        SliceMut!();
        RingBuffer!();
    };
}

macro_rules! impl_200 {
    () => {
        deps!();
        impl < 'a , A : Debug + 'a , const N : usize > Debug for SliceMut < 'a , A , N > { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { f . write_str ("RingBuffer") ? ; f . debug_list () . entries (self . iter ()) . finish () } }
    };
}

impl_200!()