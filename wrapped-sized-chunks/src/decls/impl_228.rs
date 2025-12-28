macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! impl_228 {
    () => {
        deps!();
        impl < A : Debug , const N : usize > Debug for RingBuffer < A , N > { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { f . write_str ("RingBuffer") ? ; f . debug_list () . entries (self . iter ()) . finish () } }
    };
}

impl_228!();