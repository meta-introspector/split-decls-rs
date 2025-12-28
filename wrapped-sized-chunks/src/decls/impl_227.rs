macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! impl_227 {
    () => {
        deps!();
        impl < 'a , A : Clone + 'a , const N : usize > Extend < & 'a A > for RingBuffer < A , N > { # [inline] fn extend < I : IntoIterator < Item = & 'a A > > (& mut self , iter : I) { for item in iter { self . push_back (item . clone ()) ; } } }
    };
}

impl_227!()